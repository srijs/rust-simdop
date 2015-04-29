extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSE2;

impl Set1<M8<i16>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i16) -> M8<i16> {
    Twice{
      lo: Twice{
        lo: Twice{lo: a, hi: a},
        hi: Twice{lo: a, hi: a}
      },
      hi: Twice{
        lo: Twice{lo: a, hi: a},
        hi: Twice{lo: a, hi: a}
      }
    }
  }
}

impl Add<M8<i16>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl Shli<M8<i16>> for SSE2 {
#[inline(always)]
  fn shli(&self, a: M8<i16>, i: i32) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_w(a.unwrap(), i))
    }
  }
}

impl Shri<M8<i16>> for SSE2 {
#[inline(always)]
  fn shri(&self, a: M8<i16>, i: i32) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_w(a.unwrap(), i))
    }
  }
}
