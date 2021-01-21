use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ClientMessage {
    SignIn { nickname: Nickname },
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
pub struct Nickname(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tag(u16);

impl Tag {
    pub fn gen(rng: &mut oorandom::Rand32) -> Self {
        Tag(rng.rand_range(0..(9999 + 1)) as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserId(u64);

impl UserId {
    pub fn gen(rng: &mut oorandom::Rand64) -> Self {
        Self(rng.rand_u64())
    }
}
