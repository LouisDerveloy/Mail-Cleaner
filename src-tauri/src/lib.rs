// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel, IpcResponse};

use std::fmt::{Debug, Display};
use std::future::Future;
use std::ops::DerefMut;
use crate::email_access_provider::{Credentials, EmailAccessProvider, EmailProvider, MailServer, OAuthCredentials, Sender};
use crate::utils::{CommandResult, FailureType};
use std::sync::{Mutex, MutexGuard};
use tauri::{AppHandle, Emitter, Manager, State};

mod email_access_provider;
mod utils;

#[tauri::command]
async fn get_list(state: State<'_, Mutex<AppState>>, app_handle: AppHandle, ret_channel: Channel<Vec<Sender>>, query: String) -> CommandResult {

    match state.lock() {
        Err(err) => Err(FailureType::UnknownError(err.to_string())),

        Ok(mut _state) => {
            match (*_state).email_session {
                None => {
                    app_handle.emit("open-login-page", "").expect("Couldn't emit open-login-page");
                    Err(FailureType::NotConnected)
                }
                Some(ref mut session) => session.get_unique_senders_email_list(query, ret_channel)
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
async fn token_connect(state: State<'_, Mutex<AppState>>, server: String, port: u16,email: String, token: String) -> CommandResult {
    // println!("server: {}", server);
    // println!("port: {}", port.to_string());
    // println!("email: {}", email);
    // println!("token: {}", token);

    let provider = MailServer::new(server, port);
    let cred = Credentials::Oauth(OAuthCredentials::new(email, token));

    let email_session = EmailAccessProvider::new(provider.clone(), cred);

    match state.lock() {
        Ok(mut _state) => {
            _state.mail_server = Some(provider);
            _state.email_session = Some(email_session);

        }
        Err(_) => return Err(FailureType::FailedToLockState)
    }

    Ok(())
}

#[derive(Default)]
struct AppState {
    email_session: Option<EmailAccessProvider>,
    mail_server: Option<MailServer>
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_list, test, token_connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
