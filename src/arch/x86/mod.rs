use std::option::Option;

use super::X86;
use super::Token;

enum X86Token { Token }

impl X86 {
  pub fn detect() -> Option<X86> {
    if cfg!(target_arch = "x86") {
      Option::Some(X86(Token::Token))
    } else if cfg!(target_arch = "x86_64") {
      Option::Some(X86(Token::Token))
    } else {
      Option::None
    }
  }
}

pub struct SSE2(X86Token);

impl SSE2 {
  pub fn detect(_: X86) -> Option<SSE2> {
    Option::Some(SSE2(X86Token::Token))
  }
}

pub mod sse2;
