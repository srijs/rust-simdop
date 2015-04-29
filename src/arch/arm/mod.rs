use std::option::Option;

use super::Token;

pub struct NEON(Token);

impl NEON {
  pub fn detect() -> Option<NEON> {
    Option::Some(NEON(Token::Token))
  }
}

pub mod neon;
