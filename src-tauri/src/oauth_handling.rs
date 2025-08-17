use std::fs;
use std::{collections::HashMap, fs::read_to_string};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::{Value};

use oauth2;
use webbrowser;

use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use http_body_util::Full;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use std::net;
use std::convert::Infallible;
use tokio::sync::{ oneshot, Mutex };
use std::sync::{Arc};
use oauth2::{AuthorizationCode, TokenResponse};
use tokio::time::{timeout, Duration};

use reqwest;

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

fn parse_client_secret_json<P: AsRef<Path>>(file_uri: P) -> serde_json::Result<Value> {

    let json = read_to_string(file_uri).expect("Couldn't read the file");

    println!("JSON: \n{}", json.trim());

    serde_json::from_str::<Value>(&json.trim())
}

pub async fn get_token() -> String {
    let client_secret_path = String::from("client_secret.json");

    if !(fs::exists(client_secret_path.clone()).expect("Coudn't verify the existence of the credentials.")) {
        println!("Couldn't find the credentials at {}", client_secret_path);
        "no token".into()
    } else {
        let client_secret = parse_client_secret_json(client_secret_path).expect("An error occured. While parsing the client_secret");

        // Create the http server that will catch the callback from the auth_url request to google servers
        // Create listener for TCP connexion
        let addr = net::SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = TcpListener::bind(&addr).await.expect("Couldn't create TCP listener on 127.0.0.1:0");
        let port: u16 = listener.local_addr().expect("Couldn't retrieve local ip used").port();

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


        // OAuth authentication
        let client = oauth2::basic::BasicClient::new(get_client_id(&client_secret))
            .set_client_secret(get_client_secret(&client_secret))
            .set_auth_uri(get_auth_uri(&client_secret))
            .set_token_uri(get_token_uri(&client_secret))
            .set_redirect_uri(get_redirect_uri(&client_secret, port));

        // Generate a PKCE challenge. (Proof Key for Code Exchange)
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        let (auth_url, csrf_token) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            // Set the desired scopes.
            .add_scope(oauth2::Scope::new("https://mail.google.com".to_string()))
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        println!("Redirect to: {}", auth_url);
        webbrowser::open(&auth_url.as_str()).unwrap_or_else(|e| {
            println!("Couldn't open the URL: {}. {}", auth_url, e);
            println!("Please to c ontinue the authentication process please manually go to the url.")
        });

        // Waiting for the google auth screen callback with a timeout of 10 minutes (600secs)
        let callback = timeout(Duration::from_secs(600), rx)
            .await
            .expect("Timeout en attente du callback (180s)")
            .expect("Callback interrompu sans code");

        // Stop the server
        server_task.abort();

        println!("csrf_token: {:?}", csrf_token.clone().into_secret());
        println!("google_csrf_token: {:?}", callback.state);

        if callback.state.is_none() {
            eprintln!("Error: state CSRF wasn't returned by the server");
            eprintln!("Please try again. If the problem stays, contact the developer.");
            std::process::exit(1);
        }

        if csrf_token.into_secret() != callback.state.expect("CSRF missing from the google callback.") {
            eprintln!("Error: state CSRF doesn't match");
            eprintln!("Please try again. If the problem stays, contact the developer.");
            std::process::exit(1);
        }

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
                eprintln!("Error: {}", e);
                eprintln!("Couldn't exchange code. For an authorisation token");
                std::process::exit(1);
            }
        };

        println!("Token: {}", final_tokens.access_token().secret());

        final_tokens.access_token().secret().to_string()
    }
}

fn get_redirect_uri(client_secret: &Value, port: u16) -> oauth2::RedirectUrl {
    let mut redirect_uri = client_secret
        .get("installed").unwrap()
        .get("redirect_uris").unwrap()
        .as_array().unwrap()
        .first().unwrap()
        .to_string();
    redirect_uri = redirect_uri.trim().trim_matches('"').into();
    redirect_uri = format!("{}:{}", redirect_uri, port);
    oauth2::RedirectUrl::new(
        redirect_uri
    ).expect("Couldn't create oauth::RedirectUrl")
}

fn get_token_uri(client_secret: &Value) -> oauth2::TokenUrl {
    let mut token_uri = client_secret
        .get("installed").unwrap()
        .get("token_uri").unwrap()
        .to_string();
    token_uri = token_uri.trim().trim_matches('"').into();
    oauth2::TokenUrl::new(
        token_uri
    ).expect("Couldn't create oauth2::TokenUrl")
}

fn get_auth_uri(client_secret: &Value) -> oauth2::AuthUrl {
    let mut auth_uri = client_secret
        .get("installed").unwrap()
        .get("auth_uri").unwrap()
        .to_string();
    auth_uri = auth_uri.trim().trim_matches('"').into();
    oauth2::AuthUrl::new(auth_uri)
        .expect("[get_auth_uri] Couldn't create oauth::AuthUrl")
}

fn get_client_secret(client_secret: &Value) -> oauth2::ClientSecret {
    let mut client_secret = client_secret.get("installed").unwrap()
        .get("client_secret").unwrap()
        .to_string();
    client_secret = client_secret.trim().trim_matches('"').into();
    oauth2::ClientSecret::new(
        client_secret,
    )
}

fn get_client_id(client_secret: &Value) -> oauth2::ClientId {
    let mut client_id = client_secret.get("installed").unwrap()
        .get("client_id").unwrap()
        .to_string();
    client_id = client_id.trim().trim_matches('"').into();
    oauth2::ClientId::new(
        client_id
    )
}