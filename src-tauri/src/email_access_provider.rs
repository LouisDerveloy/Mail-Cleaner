use std::fmt::{Debug, Display};
use std::net::TcpStream;
use imap;
use imap::Session;
use native_tls;
use native_tls::TlsStream;
use serde::Serialize;

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
    async fn get_inbox_senders_email_list<E>(&mut self) -> Result<Vec<String>, E>
    where E: Display + Debug;
}

pub struct EmailAccessProvider {
    mail_server: MailServer,
    imap_session: Session<TlsStream<TcpStream>>,
}

#[derive(Serialize)]
pub struct Sender {
    name: Option<String>,
    email: String,
}

impl EmailAccessProvider {
    pub fn new<E>(mail_server: MailServer, credentials: Credentials) -> Result<Self, E>
    where E: Display + Debug {
        let tls = native_tls::TlsConnector::builder().build().unwrap();

        let client = imap::connect((mail_server.domain.clone(), 993), &mail_server.domain, &tls).unwrap();

        // Login to the server based on what credentials where chosen by the user.
        let mut imap_session: Session<TlsStream<TcpStream>> = match credentials {
            Credentials::Password(credentials) => client.login(credentials.user, credentials.password).unwrap(),
            Credentials::Oauth(credentials) => client.authenticate("XOAUTH2", &credentials, ).unwrap(),
        };


        Ok(EmailAccessProvider {
            imap_session,
            mail_server,
        })
    }
}

impl EmailProvider for EmailAccessProvider {
    async fn get_inbox_senders_email_list<E>(&mut self) -> Result<Vec<Sender>, E>
    where E: Display + Debug {

        let mut senders: Vec<Sender> = Vec::new();

        // Search for all the ids of emails which match the request <BODY unsubscribe> that mean the body has to contain the word unsubscribe
        for seq in self.imap_session.search("BODY unsubscribe").unwrap().iter() {

            // Fetch the sender of those mails
            match self.imap_session.fetch(seq.to_string(), "ALL") {
                Ok(msgs) => {
                    msgs.iter().for_each(|msg| {

                        if let Some(envelop) = msg.envelope() {
                            if let Some(_senders) = &envelop.sender {
                                _senders.iter().for_each(|sender| {

                                    // Check if mailbox and host are defined
                                    if let (Some(mailbox), Some(host)) = (sender.mailbox, sender.host) {

                                        senders.push(
                                            Sender {
                                                name: sender.name.map(|name| String::from_utf8(name.to_owned()).expect("Failed to convert &[u8] of the name to String")), // .map convert Option(&[u8]) to Option(String)
                                                email: format!(
                                                    "{}@{}",
                                                    String::from_utf8(mailbox.to_vec()).expect("Failed to convert &[u8] of the mailbox to String"),
                                                    String::from_utf8(host.to_vec()).expect("Failed to convert &[u8] of the host to String")
                                                ), // Concatenate the mailbox and the host into a real mail
                                            }
                                        );

                                    }

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

        Ok(vec![])

    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}