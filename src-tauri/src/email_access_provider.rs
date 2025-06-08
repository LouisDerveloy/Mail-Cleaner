use std::fmt::{Debug, Display};
use std::collections::HashSet;
use imap;
use imap::{Connection, Session};
use serde::Serialize;
use tauri::ipc::Channel;
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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<Vec<Sender>>) -> CommandResult<Vec<String>>;
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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<Vec<Sender>>) -> CommandResult<Vec<String>> {

        println!("Get inbox senders' address from query: {}", query);

        // Search for all the ids of emails which match the request <BODY unsubscribe> that mean the body has to contain the word unsubscribe
        let search_result = match self.imap_session.search(query) {
            Ok(ids) => ids,
            Err(err) => {
                return Err(FailureType::UnknownError(err.to_string()));
            }
        };

        println!("Found {} emails matching the request.", search_result.len());

        let mut emails_list: Vec<String> = Vec::with_capacity(search_result.len());
        let mut senders: Vec<Sender> = Vec::new();
        let mut seen_emails: HashSet<String> = HashSet::new();

        for seq in search_result {

            // Fetch the sender of those mails
            let result = self.imap_session.fetch(seq.to_string(), "ALL"); // TODO: We probably don't need to select ALL but maybe only HEADER or HEADER[SENDER].

            match result {

                Ok(msgs) => {

                    for msg in msgs.iter() {

                        if let Some(envelop) = msg.envelope() {
                            if let Some(_senders) = &envelop.sender {

                                for sender in _senders {

                                    // Check if mailbox and host are defined
                                    if let (Some(mailbox), Some(host)) = (sender.mailbox.clone(), sender.host.clone()) {


                                        /*
                                        Maybe we should store each sender in a vector so the sender's id is the sender's index in the vector. So when the frontend send us a list of sender we need to erase, it just has to pass a list of id and not the whole sender email.
                                        Therefore we could retrieve the entire email from the vector. These would cause much shorter messages to be sent from the frontend.

                                        This is an example of how we could delete all the messages from a sender:
                                            // 2. Select INBOX
                                            session.select("INBOX")?;

                                            // 3. Search for emails from the sender
                                            let search_criteria = format!("FROM \"{}\"", sender);
                                            let messages = session.search(search_criteria)?;

                                            if messages.is_empty() {
                                                println!("No emails found from this sender.");
                                            } else {
                                                // 4. Mark each email as \Deleted
                                                for uid in messages.iter() {
                                                    session.store(format!("{}", uid), "+FLAGS (\\Deleted)")?;
                                                }

                                                // 5. Expunge to delete them permanently
                                                session.expunge()?;
                                                println!("Deleted {} emails from {}", messages.len(), sender);
                                            }

                                            // Logout
                                            session.logout()?;
                                            Ok(())
                                        }
                                        */

                                        let email = format!(
                                            "{}@{}",
                                            String::from_utf8(mailbox.to_vec()).expect("Failed to convert &[u8] of the mailbox to String"),
                                            String::from_utf8(host.to_vec()).expect("Failed to convert &[u8] of the host to String")
                                        );

                                        // Check if we have already processed this email address.
                                        // `seen_emails.insert()` returns true only if the email is new to the set.
                                        if seen_emails.insert(email.clone()) {
                                            emails_list.push(email.clone());
                                            let sender_id = (emails_list.len() - 1) as u32;

                                            let _name = match &sender.name {
                                                Some(name) => Some(String::from_utf8_lossy(name.as_ref()).to_string()),
                                                None => None,
                                            };


                                            senders.push(Sender {
                                                id: sender_id,
                                                name: _name,
                                                email,
                                            });
                                            println!("{} senders", senders.len());

                                            if senders.len() >= 10 {
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

        Ok(emails_list)
    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}