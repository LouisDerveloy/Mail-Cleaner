// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel, IpcResponse};

use std::fmt::{Debug, Display};
use std::future::Future;
use std::net::TcpStream;
use imap;
use native_tls;
use base64;
use imap::Session;
use native_tls::TlsStream;
use serde::Serialize;
use crate::email_access_provider::{EmailAccessProvider, EmailProvider, MailServer, OAuthCredentials, Sender};
mod email_access_provider;

#[derive(Serialize)]
enum CommandResult {
    Success,
    Failure(String),
}


#[tauri::command]
async fn get_list(ret_channel: Channel<Sender>) -> CommandResult {

    // let gmail_auth = email_access_provider::OAuthCredentials::new(
    //     String::from("louis.derveloy.gta5@gmail.com"),
    //     String::from("ya29.a0AZYkNZh7GX2BSTgksTcgZTXENVXw8KVbUi_3vwSdoxrEc1krnaKph5GIhPaen_os_tkieM8VsVcqtMuvXq2gA2v_ZERqySS_32a8LO238R5KQ8izg9HPFZr8tqRMqwCiBTLY5jYS4xE3E-5aerS9I5TOmF1iGQxhMrrVqYPkaCgYKASESARESFQHGX2Mi-Nv9o8CIB3TexAx4jFkxTg0175"),
    // );
    //
    // let domain = "imap.gmail.com";
    // let tls = native_tls::TlsConnector::builder().build().unwrap();
    //
    // // we pass in the domain twice to check that the server's TLS
    // // certificate is valid for the domain we're connecting to.
    // let client = imap::connect((domain, 993), domain, &tls).unwrap();
    //
    // // the client we have here is unauthenticated.
    // // to do anything useful with the e-mails, we need to log in
    //
    // let mut imap_session = client.authenticate(
    //     "XOAUTH2",
    //     &gmail_auth,
    // ).expect("Authentication failed");
    //
    //
    //
    // for name in imap_session.list(None, None).expect("List failed").iter() {
    //     println!("Name: {:?}", name);
    // };
    //
    //
    // // we want to fetch the first email in the INBOX mailbox
    // let number_of_emails = imap_session.select("INBOX").expect("select failed").exists;
    // println!("Number of emails: {}", number_of_emails);
    //
    //
    // // Search for emails
    // for seq in imap_session.search("BODY reddit").unwrap().iter() {
    //     match imap_session.fetch(seq.to_string(), "ALL") {
    //         Ok(msgs) => {
    //             msgs.iter().for_each(|msg| {
    //
    //                 if let Some(envelop) = msg.envelope() {
    //                     if let Some(senders) = &envelop.sender {
    //                         senders.iter().for_each(|sender| {
    //
    //                             println!("Mailbox: {:?}\n\n", match sender.mailbox {
    //                                 None => "".to_string(),
    //                                 Some(host) => String::from_utf8(host.to_vec()).unwrap(),
    //                             });
    //                             println!("Host: {:?}", match sender.host {
    //                                 None => "".to_string(),
    //                                 Some(host) => String::from_utf8(host.to_vec()).unwrap(),
    //                             });
    //
    //                             println!("Name: {:?}", match sender.name {
    //                                 None => "".to_string(),
    //                                 Some(host) => String::from_utf8(host.to_vec()).unwrap(),
    //                             });
    //
    //                         })
    //                     }
    //                 }
    //
    //             })
    //         }
    //         Err(err) => {
    //             println!("Couldn't fetch\nError: {:?}", err);
    //         }
    //     }
    // }

    let email_session = EmailAccessProvider::new(
        MailServer::new(
            String::from("imap.gmail.com"),
            993,
        ),
        email_access_provider::Credentials::Oauth(OAuthCredentials::new(
            String::from("louis.derveloy.gta5@gmail.com"),
            String::from("ya29.a0AZYkNZgbWbXjx6Rh1gUdRPXqQ8Ig7OzuZdcq1-uXBbmsk1eMbzBH8ei_MJ8yJqlVVjlgYshX_dnbeY10G-gXYQ36VY7CFVVLenxKvHWLHOMx4QDtxY8oeBne-vDLnzALl5v12Xh7Agz-i5NVOltRK-ImD1b-tkODZG_qcQqaaCgYKAdoSARESFQHGX2Mib0cDiHrGElF8DmmfcSooUQ0175"),
        ))
    );

    if email_session.is_err() {
        return CommandResult::Failure(format!("An error occurred: {}", email_session.err().unwrap()));
    }

    let mut email_session = email_session.unwrap();

    let senders: Vec<Sender> = email_session.get_inbox_senders_email_list().await.expect("Couldn't get senders' emails");

    for sender in senders.into_iter() {
        ret_channel.send(sender).unwrap();
    }

    CommandResult::Success
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
