// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel, IpcResponse};

use std::fmt::{Debug, Display};
use std::future::Future;
use imap;
use native_tls;
use base64;
use serde::Serialize;
use crate::email_access_provider::{EmailAccessProvider, EmailProvider, MailServer, OAuthCredentials, Sender};
use std::sync::{Mutex, MutexGuard};
use tauri::{Manager, State};

mod email_access_provider;

#[derive(Serialize)]
enum CommandResult {
    Success,
    Failure(FailureType),
}

#[derive(Serialize)]
enum FailureType {
    NoSenderFound,
    FailedToLockState,
    UnknownError(String),
}


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
fn test(state: State<'_, Mutex<AppState>>) -> CommandResult {
    let guard: MutexGuard<AppState> = match state.lock().ok() {
        None => {
            return CommandResult::Failure(FailureType::FailedToLockState);
        },
        Some(data) => data
    };

    if guard.email_session.is_none() {
        println!("email session is none");
        println!("Please provide an email session")
    } else {
        println!("email session is not none");
    }

    CommandResult::Success
}

#[tauri::command]
fn token_connect(state: State<'_, Mutex<AppState>>, email: String, token: String) -> CommandResult {
    println!("email: {}", email);
    println!("token: {}", token);
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
