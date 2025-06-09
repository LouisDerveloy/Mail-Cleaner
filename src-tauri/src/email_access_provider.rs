use std::fmt::{Debug, Display};
use std::collections::HashSet;
use imap;
use imap::{Connection, Session};
use serde::Serialize;
use tauri::ipc::Channel;
use crate::utils::{CommandResult, FailureType};
use std::collections::HashMap;

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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<Progress>) -> CommandResult<Vec<Sender>>;
    fn delete_senders(&mut self, sender_emails: Vec<String>, ret_channel: Channel<Progress>) -> CommandResult;
}

pub struct EmailAccessProvider {
    imap_session: Session<Connection>,
}

#[derive(Serialize, Clone)]
pub struct Sender {
    pub id: u32,
    pub name: Option<String>,
    pub email: String,
    pub occurrence: u32,
}

#[derive(Serialize, Clone)]
pub struct Progress {
    pub current: u32,
    pub total: u32,
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
    fn get_unique_senders_email_list(&mut self, query: String, ret_channel: Channel<Progress>) -> CommandResult<Vec<Sender>> {

        println!("Get inbox senders' address from query: {}", query);

        // Search for all the ids of emails which match the request <BODY unsubscribe> that mean the body has to contain the word unsubscribe
        let search_result = self.imap_session.search(query).map_err(|e| FailureType::UnknownError(e.to_string()))?;

        let total_emails = search_result.len() as u32;
        println!("Found {} emails matching the request.", total_emails);

        let mut senders_map: HashMap<String, Sender> = HashMap::new();

        for (index, seq) in search_result.iter().enumerate() {
            // Fetch the sender of those mails
            let result = self.imap_session.fetch(seq.to_string(), "ALL"); // Try with "ENVELOPE" to get the sender name

            if let Ok(msgs) = result {
                for msg in msgs.iter() {
                    if let Some(envelop) = msg.envelope() {
                        if let Some(_senders) = &envelop.sender {
                            for sender in _senders {
                                // Check if mailbox and host are defined
                                if let (Some(mailbox), Some(host)) = (sender.mailbox.as_ref(), sender.host.as_ref()) {
                                    let email = format!(
                                        "{}@{}",
                                        String::from_utf8_lossy(mailbox),
                                        String::from_utf8_lossy(host)
                                    );
                                    let _name = sender.name.as_ref().map(|s| String::from_utf8_lossy(s.as_ref()).to_string());

                                    let sender_entry = senders_map.entry(email.clone()).or_insert_with(|| Sender {
                                        id: 0, // Placeholder, will be set later
                                        name: _name,
                                        email,
                                        occurrence: 0,
                                    });
                                    sender_entry.occurrence += 1;
                                }
                            }
                        }
                    }
                }
            }

            if (index + 1) % 20 == 0 || (index + 1) as u32 == total_emails {
                let progress = Progress { current: (index + 1) as u32, total: total_emails };
                ret_channel.send(progress).unwrap();
            }
        }

        let mut final_senders: Vec<Sender> = senders_map.into_values().collect();
        final_senders.sort_by_key(|s| s.occurrence);
        final_senders.reverse();
        for (i, sender) in final_senders.iter_mut().enumerate() {
            sender.id = i as u32;
        }

        Ok(final_senders)
    }

    fn delete_senders(&mut self, sender_emails: Vec<String>, ret_channel: Channel<Progress>) -> CommandResult {
        let mut uids_to_delete = std::collections::BTreeSet::new();
        let mut progress = Progress { current: 0, total: 0 };

        for sender_email in &sender_emails {
            println!("Searching emails from: {}", sender_email);

            let search_criteria = format!("FROM \"{}\"", sender_email);
            let messages = self.imap_session.search(search_criteria)
                .map_err(|e| FailureType::UnknownError(e.to_string()))?;

            for uid in messages.iter() {
                if uids_to_delete.insert(*uid) {
                    progress.total += 1;
                }
            }
            ret_channel.send(progress.clone()).unwrap();
        }


        if uids_to_delete.is_empty() {
            println!("No emails to delete.");
            return Ok(());
        }

        let uids_vec: Vec<u32> = uids_to_delete.into_iter().collect();

        // Batch deletion
        for chunk in uids_vec.chunks(100) {
            let sequence_set = chunk
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",");

            if !sequence_set.is_empty() {
                println!("Marking {} emails for deletion.", chunk.len());
                self.imap_session.store(&sequence_set, "+FLAGS (\\Deleted)")
                    .map_err(|e| FailureType::UnknownError(e.to_string()))?;
                
                progress.current += chunk.len() as u32;
                ret_channel.send(progress.clone()).unwrap();
            }
        }

        println!("Expunging marked emails.");
        self.imap_session.expunge()
            .map_err(|e| FailureType::UnknownError(e.to_string()))?;
        println!("Successfully expunged emails.");
        
        // Ensure final progress is sent.
        progress.current = progress.total;
        ret_channel.send(progress.clone()).unwrap();

        Ok(())
    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}