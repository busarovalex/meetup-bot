use super::{User, DateTime};

#[derive(Debug)]
pub struct IncomingMessage {
    pub text: String,
    pub time: DateTime,
    pub from: User
}

#[derive(Debug)]
pub struct OutgoingMessage {
    pub text: String
}
