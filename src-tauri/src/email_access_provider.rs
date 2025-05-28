use std::fmt::{Debug, Display};
use std::sync::Mutex;
use imap;
use imap::{Connection, Session};
use native_tls;
use serde::Serialize;
use tauri::{AppHandle, State};
use crate::AppState;

pub fn get_email_session(state: State<'_, Mutex<AppState>>, app_handle: AppHandle) -> Option<EmailAccessProvider> {
    /*
    TODO: Check if email_session is already in the states.
    TODO: if already in the states just return a reference to it otherwise :
    TODO: Return an error typed
    so before that we need to create a command to initialise a email_session from the frontend
    */

    None
}

pub struct OAuthCredentials {
    user: String,
    access_token: String,
}

impl OAuthCredentials {
    pub fn new(user: String, access_token: String) -> OAuthCredentials {
        OAuthCredentials { user, access_token }
    }
}

impl imap::Authenticator for OAuthCredentials {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

pub struct PasswordCredentials {
    user: String,
    password: String,
}

impl PasswordCredentials {
    pub fn new(user: String, password: String) -> PasswordCredentials {
        PasswordCredentials { user, password }
    }
}

pub enum Credentials {
    Password(PasswordCredentials),
    Oauth(OAuthCredentials),
}

#[derive(Clone)]
pub struct MailServer {
    domain: String,
    port: u16,
}

impl MailServer {
    pub fn new(domain: String, port: u16) -> MailServer {
        MailServer { domain, port, }
    }
}

pub trait EmailProvider {
    fn get_inbox_senders_email_list(&mut self, query: String) -> Vec<Sender>;
}

pub struct EmailAccessProvider {
    imap_session: Session<Connection>,
}

#[derive(Serialize)]
pub struct Sender {
    id: u32,
    name: Option<String>,
    email: String,
}

impl EmailAccessProvider {
    pub fn new(mail_server: MailServer, credentials: Credentials) -> Self {

        let client = imap::ClientBuilder::new(mail_server.domain, mail_server.port).connect().unwrap();

        // Login to the server based on what credentials where chosen by the user.
        let mut imap_session: Session<Connection> = match credentials {
            Credentials::Password(credentials) => client.login(credentials.user, credentials.password).unwrap(),
            Credentials::Oauth(credentials) => client.authenticate("XOAUTH2", &credentials, ).unwrap(),
        };

        imap_session.select("INBOX").unwrap();

        EmailAccessProvider {
            imap_session,
        }
    }
}

impl EmailProvider for EmailAccessProvider {
    fn get_inbox_senders_email_list(&mut self, query: String) -> Vec<Sender> {

        println!("Get inbox senders' address from query: {}", query);

        let mut senders: Vec<Sender> = Vec::new();

        // Search for all the ids of emails which match the request <BODY unsubscribe> that mean the body has to contain the word unsubscribe
        let search_result = match self.imap_session.search(query) {
            Ok(ids) => ids,
            Err(err) => {
                eprintln!("No email found according to the request. Error: {:?}", err);
                return senders;
            }
        };

        println!("Found {} emails matching the request.", search_result.len());


        for seq in search_result {

            // Fetch the sender of those mails
            let result = self.imap_session.fetch(seq.to_string(), "ALL");

            match result {

                Ok(msgs) => {

                    for msg in msgs.iter() {

                        if let Some(envelop) = msg.envelope() {
                            if let Some(_senders) = &envelop.sender {
                                for sender in _senders {

                                    // Check if mailbox and host are defined
                                    if let (Some(mailbox), Some(host)) = (sender.mailbox.clone(), sender.host.clone()) {

                                        let _name = match &sender.name {
                                            Some(name) => Some(String::from_utf8_lossy(name.as_ref()).to_string()),
                                            None => None,
                                        };

                                        senders.push(
                                            Sender {
                                                id: seq,
                                                name: _name,
                                                email: format!(
                                                    "{}@{}",
                                                    String::from_utf8(mailbox.to_vec()).expect("Failed to convert &[u8] of the mailbox to String"),
                                                    String::from_utf8(host.to_vec()).expect("Failed to convert &[u8] of the host to String")
                                                ), // Concatenate the mailbox and the host into a real mail
                                            }
                                        );

                                        println!("mail sent to frontend")
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Couldn't fetch\nError: {:?}", err);
                }
            }
        }

        senders
    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}