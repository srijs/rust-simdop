use std::option::Option;

use super::Token;

pub struct SSE2(Token);

impl SSE2 {
  pub fn detect() -> Option<SSE2> {
    Option::Some(SSE2(Token::Token))
  }
}

pub struct SSSE3(Token);

impl SSSE3 {
  pub fn detect() -> Option<SSSE3> {
    Option::Some(SSSE3(Token::Token))
  }
}

pub mod sse2;
pub mod ssse3;
