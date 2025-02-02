use crate::prelude::*;
use std::num::NonZeroU128;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Token(NonZeroU128);

impl Token {
    pub fn new(value: NonZeroU128) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: Token,
}
