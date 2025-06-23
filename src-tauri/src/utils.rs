use serde::Serialize;
use serde::Deserialize;

pub type CommandResult<T = ()> = Result<T, FailureType>;

#[derive(Serialize)]
pub enum FailureType {
    NoSenderFound,
    ImapConnectionError(String),
    FailedToLockState,
    NotConnected,
    UnknownError(String),
    IDDidntMatch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub text: Option<String>,
    pub body: Option<String>,
    pub before: Option<String>,
    pub cc: Option<String>,
    pub from: Option<String>,
    pub new_: bool,
    pub since: Option<String>,
    pub subject: Option<String>,
    pub to: Option<String>,
    pub unseen: bool,
    pub seen: bool,
}

pub fn constuct_query(search: Search) -> String {
    let mut query_parts = Vec::new();

    if let Some(ref val) = search.text {
        if !val.is_empty() {
            query_parts.push(format!("TEXT {}", val));
        }
    }
    if let Some(ref val) = search.body {
        if !val.is_empty() {
            query_parts.push(format!("BODY {}", val));
        }
    }
    if let Some(ref val) = search.before {
        if !val.is_empty() {
            query_parts.push(format!("BEFORE {}", val));
        }
    }
    if let Some(ref val) = search.cc {
        if !val.is_empty() {
            query_parts.push(format!("CC {}", val));
        }
    }
    if let Some(ref val) = search.from {
        if !val.is_empty() {
            query_parts.push(format!("FROM {}", val));
        }
    }
    if search.new_ {
        query_parts.push("NEW".to_string());
    }
    if let Some(ref val) = search.since {
        if !val.is_empty() {
            query_parts.push(format!("SINCE {}", val));
        }
    }
    if let Some(ref val) = search.subject {
        if !val.is_empty() {
            query_parts.push(format!("SUBJECT {}", val));
        }
    }
    if let Some(ref val) = search.to {
        if !val.is_empty() {
            query_parts.push(format!("TO {}", val));
        }
    }
    if search.unseen {
        query_parts.push("UNSEEN".to_string());
    }
    if search.seen {
        query_parts.push("SEEN".to_string());
    }

    if query_parts.is_empty() {
        "ALL".to_string()
    } else {
        query_parts.join(" ")
    }
}