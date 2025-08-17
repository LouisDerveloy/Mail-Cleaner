// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel};
use std::sync::{Mutex, MutexGuard};
use oauth2::RefreshToken;
use tauri::{AppHandle, Emitter, Manager, State};

mod email_access_provider;
use crate::email_access_provider::{Credentials, EmailAccessProvider, EmailProvider, MailServer, OAuthCredentials, PasswordCredentials, Sender, Progress};
mod utils;
use crate::utils::{CommandResult, FailureType, constuct_query, Search};

mod oauth_handling;

#[tauri::command]
async fn get_list(state: State<'_, Mutex<AppState>>, app_handle: AppHandle, ret_channel: Channel<Progress>, query: Search) -> CommandResult<Vec<Sender>> {

    match state.lock() {
        Err(_) => Err(FailureType::FailedToLockState),

        Ok(mut _state) => {
            match _state.email_session {
                None => {
                    app_handle.emit("open-login-page", "").expect("Couldn't emit open-login-page");
                    Err(FailureType::NotConnected)
                }
                Some(ref mut session) => {

                    // Create query
                    let query = constuct_query(query);

                    match session.get_unique_senders_email_list(query, ret_channel) {
                        Ok(emails_list) => {
                            _state.emails_list = Some(emails_list.clone());
                            Ok(emails_list)
                        },
                        Err(err) => Err(err)
                    }
                }
            }
        }
    }
}


#[tauri::command]
async fn test(state: State<'_, Mutex<AppState>>, app_handle: AppHandle) -> CommandResult {
    let guard: MutexGuard<AppState> = match state.lock().ok() {
        None => {
            return Err(FailureType::FailedToLockState);
        },
        Some(data) => data
    };

    if guard.email_session.is_none() {
        println!("email session is none");
        println!("Please provide an email session");

        app_handle.emit("open-login-page", "").expect("failed to emit open-login-page event in test command.");
    } else {
        println!("email session is not none");
    }

    Ok(())
}



#[tauri::command]
async fn delete_senders(state: State<'_, Mutex<AppState>>, sender_ids: Vec<u32>, ret_channel: Channel<Progress>) -> CommandResult {
    let mut guard = match state.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(FailureType::FailedToLockState)
    };

    let emails_to_delete: Vec<String> = if let Some(emails_list) = &guard.emails_list {
        let mut emails = Vec::new();
        for &id in sender_ids.iter() {
            if let Some(sender) = emails_list.get(id as usize) {
                if sender.id != id {
                    return Err(FailureType::IDDidntMatch);
                }
                emails.push(sender.email.clone());
            }
        }
        emails
    } else {
        return Err(FailureType::UnknownError("Email list not available".to_string()));
    };

    if emails_to_delete.is_empty() {
        return Err(FailureType::UnknownError("No valid sender emails found for given IDs.".to_string()));
    }

    if let Some(session) = &mut guard.email_session {
        session.delete_senders(emails_to_delete, ret_channel)?;
    } else {
        return Err(FailureType::NotConnected);
    }

    Ok(())
}

#[tauri::command]
async fn token_connect(state: State<'_, Mutex<AppState>>, server: String, port: u16,email: String, token: String) -> CommandResult {

    let provider = MailServer::new(server, port);
    let cred = Credentials::Oauth(OAuthCredentials::new(email, token));

    let email_session = EmailAccessProvider::new(provider.clone(), cred)?;

    match state.lock() {
        Ok(mut _state) => {
            _state.mail_server = Some(provider);
            _state.email_session = Some(email_session);

        }
        Err(_) => return Err(FailureType::FailedToLockState)
    }

    Ok(())
}

#[tauri::command]
async fn password_connect(state: State<'_, Mutex<AppState>>, server: String, port: u16, email: String, password: String) -> CommandResult {
    let provider = MailServer::new(server, port);
    let cred = Credentials::Password(PasswordCredentials::new(email, password));

    let email_session = EmailAccessProvider::new(provider.clone(), cred)?;

    match state.lock() {
        Ok(mut _state) => {
            _state.mail_server = Some(provider);
            _state.email_session = Some(email_session);
        }
        Err(_) => return Err(FailureType::FailedToLockState)
    }

    Ok(())
}

#[tauri::command]
async fn gmail_oauth_request(state: State<'_, Mutex<AppState>>) -> CommandResult {

    let (secret_token, refresh_token, email) = oauth_handling::get_token().await?;

    match state.lock() {
        Ok(mut _state) => {
            _state.refresh_token = refresh_token;
        }
        Err(_) => return Err(FailureType::FailedToLockState)
    }

    token_connect(state, "imap.gmail.com".into(), 993, email, secret_token).await
}

#[derive(Default)]
struct AppState {
    email_session: Option<EmailAccessProvider>,
    mail_server: Option<MailServer>,
    emails_list: Option<Vec<Sender>>,
    refresh_token: Option<RefreshToken>
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_list, test, token_connect, delete_senders, password_connect, gmail_oauth_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
