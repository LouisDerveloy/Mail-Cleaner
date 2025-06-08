use serde::Serialize;

pub type CommandResult<T = ()> = Result<T, FailureType>;

#[derive(Serialize)]
pub enum FailureType {
    NoSenderFound,
    ImapConnectionError(String),
    FailedToLockState,
    NotConnected,
    UnknownError(String),
}