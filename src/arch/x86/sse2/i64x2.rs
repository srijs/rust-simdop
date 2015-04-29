extern crate llvmint;

use super::super::SSE2;
use super::super::super::super::*;
use super::super::super::super::core::*;

impl Set1<M2<i64>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i64) -> M2<i64> {
    Twice{lo: a, hi: a}
  }
}

impl Add<M2<i64>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M2<i64>, b: M2<i64>) -> M2<i64> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl Shli<M2<i64>> for SSE2 {
#[inline(always)]
  fn shli(&self, a: M2<i64>, i: i32) -> M2<i64> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_q(a.unwrap(), i))
    }
  }
}

impl Shri<M2<i64>> for SSE2 {
#[inline(always)]
  fn shri(&self, a: M2<i64>, i: i32) -> M2<i64> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_q(a.unwrap(), i))
    }
  }
}
