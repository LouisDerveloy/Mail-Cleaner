use std::fmt::{Debug, Display};
use imap;
use imap::{Connection, Session};
use native_tls;
use serde::Serialize;
use tauri::ipc::Channel;
use tauri::utils::acl::Value::List;
use crate::utils::{CommandResult, FailureType};

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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<Vec<Sender>>) -> CommandResult;
}

pub struct EmailAccessProvider {
    imap_session: Session<Connection>,
}

#[derive(Serialize, Clone)]
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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<Vec<Sender>>) -> CommandResult {

        println!("Get inbox senders' address from query: {}", query);

        // Search for all the ids of emails which match the request <BODY unsubscribe> that mean the body has to contain the word unsubscribe
        let search_result = match self.imap_session.search(query) {
            Ok(ids) => ids,
            Err(err) => {
                return CommandResult::Failure(FailureType::UnknownError(err.to_string()));
            }
        };

        println!("Found {} emails matching the request.", search_result.len());

        let mut senders: Vec<Sender> = Vec::new();

        for seq in search_result {

            // Fetch the sender of those mails
            let result = self.imap_session.fetch(seq.to_string(), "ALL"); // TODO: We probably don't need to select ALL but maybe only HEADER or HEADER[SENDER].

            match result {

                Ok(msgs) => {

                    for msg in msgs.iter() {

                        if let Some(envelop) = msg.envelope() {
                            if let Some(_senders) = &envelop.sender {

                                for sender in _senders {

                                    // TODO: Check is the sender hasn't already be seen if so add 1 to counter
                                    // Maybe we could use a hashmap to keep track of which sender we saw and how many time

                                    // Check if mailbox and host are defined
                                    if let (Some(mailbox), Some(host)) = (sender.mailbox.clone(), sender.host.clone()) {

                                        let _name = match &sender.name {
                                            Some(name) => Some(String::from_utf8_lossy(name.as_ref()).to_string()),
                                            None => None,
                                        };


                                        senders.push(Sender {
                                            id: seq,
                                            name: _name,
                                            email: format!(
                                                "{}@{}",
                                                String::from_utf8(mailbox.to_vec()).expect("Failed to convert &[u8] of the mailbox to String"),
                                                String::from_utf8(host.to_vec()).expect("Failed to convert &[u8] of the host to String")
                                            ), // Concatenate the mailbox and the host into a real mail
                                        });
                                        println!("{} senders", senders.len());

                                        if senders.len() >= 20 {
                                            println!("Sending {} senders", senders.len());
                                            ret_channel.send(senders.to_vec()).unwrap();
                                            senders.clear();
                                        }
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

        // Send the remaining senders
        if senders.len() > 0 {
            ret_channel.send(senders.to_vec()).unwrap();
        }

        CommandResult::Success
    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}