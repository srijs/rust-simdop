extern crate simdty;
extern crate llvmint;

use super::super::SSE2;
use super::super::super::super::*;
use super::super::super::super::core::*;

impl Multi for M2<i64> {
  type Elem = i64;
  type Repr = simdty::i64x2;
#[inline(always)]
  fn wrap(s: Self::Repr) -> Self {
    Twice{lo: s.0, hi: s.1}
  }
#[inline(always)]
  fn unwrap(self) -> Self::Repr {
    simdty::i64x2(self.lo, self.hi)
  }
}

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
