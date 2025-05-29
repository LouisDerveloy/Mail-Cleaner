use serde::Serialize;

#[derive(Serialize)]
pub enum CommandResult {
    Success,
    Failure(FailureType),
}

#[derive(Serialize)]
pub enum FailureType {
    NoSenderFound,
    FailedToLockState,
    NotConnected,
    UnknownError(String),
}