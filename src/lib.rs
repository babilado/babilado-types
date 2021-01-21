use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ClientMessage {
    SignIn { nickname: Nickname, tag: Tag },
    Event(Event),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Event {
    NewMessage(Message),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Message {
    pub body: String,
    pub author: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct User {
    pub nickname: Nickname,
    pub tag: Tag,
    pub id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Nickname(String);

impl Nickname {
    pub fn new(nickname: String) -> Self {
        Self(nickname)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tag(u16);

impl Tag {
    pub fn new(tag: u16) -> Self {
        Self(tag)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserId(u64);

impl UserId {
    pub fn gen(rng: &mut oorandom::Rand64) -> Self {
        Self(rng.rand_u64())
    }
}
