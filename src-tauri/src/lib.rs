// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel, IpcResponse};

use std::fmt::{Debug, Display};
use imap;
use native_tls;
use base64;
// use crate::email_access_provider::EmailProvider;
// mod email_access_provider;


struct GmailOAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}


#[tauri::command]
async fn get_list(ret_channel: Channel<[i32; 10]>) {

    // let mut email_provider = email_access_provider::EmailAccessProvider::new(
    //     email_access_provider::MailServer::new(String::from("imap.gmail.com"), 993),
    //     "louis.derveloy@gmail.com",
    //     "qsdfjl"
    // ).expect("Failed to initialize email provider");
    //
    // match email_provider.get_inbox_senders_email_list().await {
    //     Ok(msg) => {
    //         println!("Email fetched msg = {}", msg.first().unwrap());
    //     }
    //     Err(e_msg) => {
    //         println!("Error getting inbox senders email list: {}", e_msg);
    //     }
    // };

    let gmail_auth = GmailOAuth2 {
        user: String::from("louis.derveloy.gta5@gmail.com"),
        access_token: String::from("ya29.a0AZYkNZh7GX2BSTgksTcgZTXENVXw8KVbUi_3vwSdoxrEc1krnaKph5GIhPaen_os_tkieM8VsVcqtMuvXq2gA2v_ZERqySS_32a8LO238R5KQ8izg9HPFZr8tqRMqwCiBTLY5jYS4xE3E-5aerS9I5TOmF1iGQxhMrrVqYPkaCgYKASESARESFQHGX2Mi-Nv9o8CIB3TexAx4jFkxTg0175"),
    };

    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in

    let mut imap_session = client.authenticate(
        "XOAUTH2",
        &gmail_auth,
    ).expect("Authentication failed");



    for name in imap_session.list(None, None).expect("List failed").iter() {
        println!("Name: {:?}", name);
    };


    // we want to fetch the first email in the INBOX mailbox
    let number_of_emails = imap_session.select("INBOX").expect("select failed").exists;
    println!("Number of emails: {}", number_of_emails);


    // Search for emails
    for seq in imap_session.search("BODY reddit").unwrap().iter() {
        match imap_session.fetch(seq.to_string(), "ALL") {
            Ok(msgs) => {
                msgs.iter().for_each(|msg| {

                    if let Some(envelop) = msg.envelope() {
                        if let Some(senders) = &envelop.sender {
                            senders.iter().for_each(|sender| {

                                println!("Mailbox: {:?}\n\n", match sender.mailbox {
                                    None => "".to_string(),
                                    Some(host) => String::from_utf8(host.to_vec()).unwrap(),
                                });
                                println!("Host: {:?}", match sender.host {
                                    None => "".to_string(),
                                    Some(host) => String::from_utf8(host.to_vec()).unwrap(),
                                });

                                println!("Name: {:?}", match sender.name {
                                    None => "".to_string(),
                                    Some(host) => String::from_utf8(host.to_vec()).unwrap(),
                                });

                            })
                        }
                    }

                })
            }
            Err(err) => {
                println!("Couldn't fetch\nError: {:?}", err);
            }
        }
    }
    //
    //
    // // fetch message number 1 in this mailbox, along with its RFC822 field.
    // // RFC 822 dictates the format of the body of e-mails
    // for i in 1u8..11u8 {
    //     println!("Fetching i={}", i);
    //
    //     match imap_session.fetch(i.to_string(), "[Gmail]/All Mail") {
    //         Ok(msgs) => {
    //             for msg in msgs.iter() {
    //                 println!("Email i={} msg: {:?}", i, msg.message);
    //                 println!("Email i={} msg: {:?}", i, msg.message.body().unwrap().deserialize::<i32>().unwrap());
    //             }
    //         }
    //         Err(e) => println!("Error Fetching email {}: {}", i, e),
    //     };
    //
    //     std::thread::sleep(std::time::Duration::from_secs(2));
    // }


    let lst = [1,2,3,4,5,6,7,8,9,10];

    for i in 0..150 {
        let mut ret = lst;
        for j in 0..10 {
            ret[j as usize] = ret[j as usize] + i*10;
        }

        println!("Data sent");
        std::thread::sleep(std::time::Duration::from_secs(1));
        ret_channel.send(ret).unwrap();
    }



}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
