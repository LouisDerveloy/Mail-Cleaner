use std::fmt::{Debug, Display};
use std::net::TcpStream;
use imap;
use imap::Session;
use native_tls;
use native_tls::TlsStream;

#[derive(Clone)]
pub struct MailServer {
    domain: String,
    port: u16,
}

impl MailServer {
    pub fn new(domain: String, port: u16) -> MailServer {
        MailServer {
            domain,
            port,
        }
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

impl EmailAccessProvider {
    pub fn new<E>(mail_server: MailServer, email: &str, password: &str) -> Result<Self, E>
    where E: Display + Debug {

        let tls = native_tls::TlsConnector::builder().build()?; // Opening

        let client = imap::connect(
            (mail_server.domain.clone(), mail_server.port.clone()),
            &mail_server.domain,
            &tls,
        )?;

        let mut imap_session = client.login(email, password)?;


        Ok(Self {
            mail_server,
            imap_session,
        })
    }
}

impl EmailProvider for EmailAccessProvider {
    async fn get_inbox_senders_email_list<E>(&mut self) -> Result<Vec<String>, E>
    where E: Display + Debug {

        let messages = self.imap_session.fetch("1", "HEADER").unwrap();
        if let Some(m) = messages.iter().next() {
            Ok(vec![m.message.to_string()])
        } else {
            Ok(Vec::new())
        }
    }
}

impl Drop for EmailAccessProvider {
    fn drop(&mut self) {
        self.imap_session.logout().expect("imap session logout failed");
    }
}