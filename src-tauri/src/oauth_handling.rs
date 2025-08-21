use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, fs::read_to_string};


use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use oauth2::{AuthorizationCode, RefreshToken, TokenResponse};
use std::convert::Infallible;
use std::net;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, Mutex};
use tokio::time::{timeout, Duration};


use crate::utils::{CommandResult, FailureType};
use tauri::ipc::Channel;

// Struct to represent client_secret.json
#[derive(Serialize, Deserialize, Debug)]
struct ClientSecret {
    installed: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthCallback {
    code: AuthorizationCode,
    state: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    email_verified: Option<bool>,
    name: Option<String>,
    picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFeedback {
    pub update_type: UpdateType,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UpdateType {
    Status,
    Link,
}

fn parse_client_secret_json<P: AsRef<Path>>(file_uri: P) -> serde_json::Result<Value> {
    let json = read_to_string(file_uri).expect("Couldn't read the file");

    log::debug!("JSON: \n{}", json.trim());

    serde_json::from_str::<Value>(json.trim())
}

pub async fn get_token(
    feedback_channel: Channel<UserFeedback>,
) -> CommandResult<(String, Option<RefreshToken>, String)> {
    // (secret_token, Option<refresh_token>, email)
    let client_secret_path = String::from("client_secret.json");

    if !(fs::exists(client_secret_path.clone())
        .expect("Coudn't verify the existence of the credentials."))
    {
        log::debug!("Couldn't find the credentials at {}", client_secret_path);
        Err(FailureType::UnknownError("No client credentials".into()))
    } else {
        let client_secret = parse_client_secret_json(client_secret_path)
            .expect("An error occured. While parsing the client_secret");

        // Create the http server that will catch the callback from the auth_url request to google servers

        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Starting callback server".into(),
            })
            .map_err(|_e| FailureType::ChannelError("Starting callback server".into()))?;

        // Create listener for TCP connexion
        let addr = net::SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = TcpListener::bind(&addr)
            .await
            .expect("Couldn't create TCP listener on 127.0.0.1:0");
        let port: u16 = listener
            .local_addr()
            .expect("Couldn't retrieve local ip used")
            .port();

        let (tx, rx) = oneshot::channel::<AuthCallback>();
        let tx = Arc::new(Mutex::new(Some(tx)));

        let server_task = {
            let tx = tx.clone();
            tokio::spawn(async move {
                // Only accept one connexion and then kill the server
                if let Ok((stream, _peer)) = listener.accept().await {
                    let service = service_fn(move |req: Request<hyper::body::Incoming>| {
                        let tx = tx.clone();
                        async move {
                            // Parse la query (?code=...&state=...)
                            let uri = req.uri().clone();
                            let query = uri.query().unwrap_or_default();
                            let parsed: Result<AuthCallback, _> = serde_urlencoded::from_str(query);

                            // Envoie le code au thread principal
                            if let Ok(cb) = parsed {
                                if let Some(sender) = tx.lock().await.take() {
                                    let _ = sender.send(cb);
                                }
                            }

                            // Répond un petit HTML et termine
                            let body = Full::new(Bytes::from_static(
                                b"<html><body>Authentication sucessfull. You can close this tab.</body></html>",
                            ));
                            Ok::<_, Infallible>(Response::new(body))
                        }
                    });

                    let io = TokioIo::new(stream);
                    let _ = http1::Builder::new().serve_connection(io, service).await;
                }
            })
        };

        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Creating OAuth client".into(),
            })
            .map_err(|_e| FailureType::ChannelError("Creating OAuth client".into()))?;

        // OAuth authentication
        let client = oauth2::basic::BasicClient::new(get_client_id(&client_secret))
            .set_client_secret(get_client_secret(&client_secret))
            .set_auth_uri(get_auth_uri(&client_secret))
            .set_token_uri(get_token_uri(&client_secret))
            .set_redirect_uri(get_redirect_uri(&client_secret, port));

        // Generate a PKCE challenge. (Proof Key for Code Exchange)
        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Generating PKCE challenge".into(),
            })
            .map_err(|_e| FailureType::ChannelError("Generating PKCE challenge".into()))?;

        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Generating authentication url and csrf token".into(),
            })
            .map_err(|_e| {
                FailureType::ChannelError("Generating authentication url and csrf token".into())
            })?;

        let (auth_url, csrf_token) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            // Set the desired scopes.
            .add_scope(oauth2::Scope::new("openid".into())) // Scope to get the openid info of the authenticated user
            .add_scope(oauth2::Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".into(),
            )) // Scope to include the email of the account used to sign in
            .add_scope(oauth2::Scope::new("https://mail.google.com".into()))
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Link,
                content: auth_url.clone().into(),
            })
            .map_err(|_e| FailureType::ChannelError("Couldn't send authentication link".into()))?;

        webbrowser::open(auth_url.as_str()).unwrap_or_else(|e| {
            log::debug!("Couldn't open the URL: {}. {}", auth_url, e);
            log::debug!(
                "Please to c ontinue the authentication process please manually go to the url."
            )
        }); // Try to automatically open the url in the user's default browser

        // Waiting for the google auth screen callback with a timeout of 10 minutes (600secs)
        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Waiting for callback".into(),
            })
            .map_err(|_e| FailureType::ChannelError("Waiting for callback".into()))?;

        let callback = timeout(Duration::from_secs(600), rx)
            .await
            .expect("Timeout en attente du callback (180s)")
            .expect("Callback interrompu sans code");

        // Stop the server
        server_task.abort();

        // Check CSRF validity
        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Checking csrf".into(),
            })
            .map_err(|_e| FailureType::ChannelError("Checking csrf".into()))?;

        if callback.state.is_none() {
            log::error!("Error: state CSRF wasn't returned by the server");
            log::error!("Please try again. If the problem stays, contact the developer.");

            return Err(FailureType::UnknownError(
                "CSRF wasn't returned by the server".into(),
            ));
        }

        if csrf_token.into_secret()
            != callback
                .state
                .expect("CSRF missing from the google callback.")
        {
            log::error!("Error: state CSRF doesn't match");
            log::error!("Please try again. If the problem stays, contact the developer.");
            return Err(FailureType::UnknownError("CSRF doesn't match".into()));
        }

        // Retrieve access_token
        feedback_channel
            .send(UserFeedback {
                update_type: UpdateType::Status,
                content: "Retrieving access token".into(),
            })
            .map_err(|_e| FailureType::ChannelError("Retrieving access token".into()))?;

        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Couldn't build HTTP client");

        let token_result = client
            .exchange_code(callback.code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await;

        let final_tokens = match token_result {
            Ok(token) => token,
            Err(e) => {
                log::error!("Error: {}", e);
                log::error!("Couldn't exchange code. For an authorisation token");
                return Err(FailureType::UnknownError(format!("Error: {}", e)));
            }
        };

        let access_token = final_tokens.access_token().secret().to_string();

        let userinfo: GoogleUserInfo = reqwest::Client::new()
            // Endpoint recommandé OIDC
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(access_token.as_str())
            .send()
            .await
            .expect("Couldn't retrieve user openid info.")
            .json::<GoogleUserInfo>()
            .await
            .expect("Couldn't deserialize user info");

        log::debug!("Google User's info : {:?}", userinfo);

        Ok((
            access_token,
            final_tokens
                .refresh_token().cloned(),
            userinfo.email,
        ))
    }
}

fn get_redirect_uri(client_secret: &Value, port: u16) -> oauth2::RedirectUrl {
    let mut redirect_uri = client_secret
        .get("installed")
        .unwrap()
        .get("redirect_uris")
        .unwrap()
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .to_string();
    redirect_uri = redirect_uri.trim().trim_matches('"').into();
    redirect_uri = format!("{}:{}", redirect_uri, port);
    oauth2::RedirectUrl::new(redirect_uri).expect("Couldn't create oauth::RedirectUrl")
}

fn get_token_uri(client_secret: &Value) -> oauth2::TokenUrl {
    let mut token_uri = client_secret
        .get("installed")
        .unwrap()
        .get("token_uri")
        .unwrap()
        .to_string();
    token_uri = token_uri.trim().trim_matches('"').into();
    oauth2::TokenUrl::new(token_uri).expect("Couldn't create oauth2::TokenUrl")
}

fn get_auth_uri(client_secret: &Value) -> oauth2::AuthUrl {
    let mut auth_uri = client_secret
        .get("installed")
        .unwrap()
        .get("auth_uri")
        .unwrap()
        .to_string();
    auth_uri = auth_uri.trim().trim_matches('"').into();
    oauth2::AuthUrl::new(auth_uri).expect("[get_auth_uri] Couldn't create oauth::AuthUrl")
}

fn get_client_secret(client_secret: &Value) -> oauth2::ClientSecret {
    let mut client_secret = client_secret
        .get("installed")
        .unwrap()
        .get("client_secret")
        .unwrap()
        .to_string();
    client_secret = client_secret.trim().trim_matches('"').into();
    oauth2::ClientSecret::new(client_secret)
}

fn get_client_id(client_secret: &Value) -> oauth2::ClientId {
    let mut client_id = client_secret
        .get("installed")
        .unwrap()
        .get("client_id")
        .unwrap()
        .to_string();
    client_id = client_id.trim().trim_matches('"').into();
    oauth2::ClientId::new(client_id)
}
