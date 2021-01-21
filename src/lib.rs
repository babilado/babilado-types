use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

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

impl Distribution<Tag> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tag {
        Tag(rng.gen_range(0..=9999))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserId(u64);

impl Distribution<UserId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UserId {
        UserId(rng.gen())
    }
}
