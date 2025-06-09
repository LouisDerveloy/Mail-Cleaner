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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<SenderBulk>) -> CommandResult<Vec<String>>;
    fn delete_senders(&mut self, sender_emails: Vec<String>) -> CommandResult;
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

#[derive(Serialize, Clone)]
pub struct Progress {
    pub current: u32,
    pub total: u32,
}

#[derive(Serialize, Clone)]
pub struct SenderBulk {
    pub progress: Progress,
    pub senders: Vec<Sender>,
}

impl EmailAccessProvider {
    pub fn new(mail_server: MailServer, credentials: Credentials) -> Result<Self, FailureType> {

        let client = imap::ClientBuilder::new(mail_server.domain, mail_server.port).connect()
            .map_err(|e| FailureType::ImapConnectionError(e.to_string()))?;

        // Login to the server based on what credentials where chosen by the user.
        let mut imap_session: Session<Connection> = match credentials {
            Credentials::Password(credentials) => client.login(credentials.user, credentials.password)
                .map_err(|(e, _)| FailureType::ImapConnectionError(e.to_string()))?,
            Credentials::Oauth(credentials) => client.authenticate("XOAUTH2", &credentials, )
                .map_err(|(e, _)| FailureType::ImapConnectionError(e.to_string()))?,
        };

        imap_session.select("INBOX").map_err(|e| FailureType::ImapConnectionError(e.to_string()))?;

        Ok(EmailAccessProvider {
            imap_session,
        })
    }
}

impl EmailProvider for EmailAccessProvider {
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<SenderBulk>) -> CommandResult<Vec<String>> {

        println!("Get inbox senders' address from query: {}", query);

        // Search for all the ids of emails which match the request <BODY unsubscribe> that mean the body has to contain the word unsubscribe
        let search_result = match self.imap_session.search(query) {
            Ok(ids) => ids,
            Err(err) => {
                return Err(FailureType::UnknownError(err.to_string()));
            }
        };

        let total_emails = search_result.len() as u32;
        println!("Found {} emails matching the request.", total_emails);

        let mut emails_list: Vec<String> = Vec::with_capacity(total_emails as usize);
        let mut senders: Vec<Sender> = Vec::new();
        let mut seen_emails: HashSet<String> = HashSet::new();

        for (index, seq) in search_result.iter().enumerate() {

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
                                                let bulk = SenderBulk {
                                                    progress: Progress { current: (index + 1) as u32, total: total_emails },
                                                    senders: senders.clone(),
                                                };
                                                ret_channel.send(bulk).unwrap();
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
        if !senders.is_empty() {
            let bulk = SenderBulk {
                progress: Progress { current: total_emails, total: total_emails },
                senders: senders.to_vec(),
            };
            ret_channel.send(bulk).unwrap();
        } else {
            // Send final progress update if there are no remaining senders to ensure 100% is reached
            let bulk = SenderBulk {
                progress: Progress { current: total_emails, total: total_emails },
                senders: Vec::new(),
            };
            ret_channel.send(bulk).unwrap();
        }

        Ok(emails_list)
    }

    fn delete_senders(&mut self, sender_emails: Vec<String>) -> CommandResult {
        let mut uids_to_delete = std::collections::BTreeSet::new();

        for sender_email in sender_emails {
            println!("Searching emails from: {}", sender_email);

            let search_criteria = format!("FROM \"{}\"", sender_email);
            let messages = self.imap_session.search(search_criteria)
                .map_err(|e| FailureType::UnknownError(e.to_string()))?;

            for uid in messages.iter() {
                uids_to_delete.insert(*uid);
            }
        }

        if uids_to_delete.is_empty() {
            println!("No emails to delete.");
            return Ok(());
        }

        let sequence_set = uids_to_delete
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if !sequence_set.is_empty() {
            println!("Marking {} emails for deletion.", sequence_set.split(',').count());
            self.imap_session.store(&sequence_set, "+FLAGS (\\Deleted)")
                .map_err(|e| FailureType::UnknownError(e.to_string()))?;

            println!("Expunging marked emails.");
            self.imap_session.expunge()
                .map_err(|e| FailureType::UnknownError(e.to_string()))?;
            println!("Successfully expunged emails.");
        }

        Ok(())
    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}