// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel, IpcResponse};

use std::fmt::{Debug, Display};
use std::future::Future;
use crate::email_access_provider::{Credentials, EmailAccessProvider, EmailProvider, MailServer, OAuthCredentials, Sender};
use crate::utils::{CommandResult, FailureType};
use std::sync::{Mutex, MutexGuard};
use tauri::{AppHandle, Emitter, Manager, State};

mod email_access_provider;
mod utils;

#[tauri::command]
fn get_list(state: State<'_, Mutex<AppState>>, ret_channel: Channel<Sender>, query: String) -> CommandResult {

    let email_session = EmailAccessProvider::new(
        MailServer::new(
            String::from("imap.gmail.com"),
            993,
        ),
        email_access_provider::Credentials::Oauth(OAuthCredentials::new(
            String::from("louis.derveloy.gta5@gmail.com"),
            String::from("ya29.a0AW4XtxhGd22JD_NENJ2FPxE-S3oUV06NCEOZpTtgGVVsnvXAJGOLGJtkkYqSxSVuT5AQvrfy5ijArzF862Rk3Dt8uR106-qR8GdchPWEwYXIHdyMDCfQiB5BzXnGXc7m8VV2JlAZ102dTe8LEClvFkqD4ueRNdDrJAjmyA1uaCgYKAbMSARESFQHGX2Miu6bcCQ9IY3QPx9aHj5tAEg0175"),
        ))
    );

    let mut email_session = email_session;

    let senders: Vec<Sender> = email_session.get_inbox_senders_email_list(query);

    if senders.is_empty() {
        CommandResult::Failure(FailureType::NoSenderFound);
    }

    for sender in senders.into_iter() {
        ret_channel.send(sender).unwrap();
    }

    CommandResult::Success
}

#[tauri::command]
fn test(state: State<'_, Mutex<AppState>>, app_handle: AppHandle) -> CommandResult {
    let guard: MutexGuard<AppState> = match state.lock().ok() {
        None => {
            return CommandResult::Failure(FailureType::FailedToLockState);
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

    CommandResult::Success
}

#[tauri::command]
fn token_connect(state: State<'_, Mutex<AppState>>, server: String, port: u16,email: String, token: String) -> CommandResult {
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
        Err(_) => return CommandResult::Failure(FailureType::FailedToLockState)
    }

    CommandResult::Success
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
