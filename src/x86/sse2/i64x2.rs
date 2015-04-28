extern crate simdty;
extern crate llvmint;

use super::super::super::*;

impl Multi for M2<i64> {
  type Elem = i64;
  type Repr = simdty::i64x2;
#[inline(always)]
  fn wrap(s: Self::Repr) -> Self {
    Twice(s.0, s.1)
  }
#[inline(always)]
  fn unwrap(self) -> Self::Repr {
    simdty::i64x2(self.0, self.1)
  }
}

impl Set1 for M2<i64> {
#[inline(always)]
  fn set1(a: Self::Elem) -> Self {
    Twice(a, a)
  }
}

impl Add for M2<i64> {
#[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Multi::wrap(self.unwrap() + rhs.unwrap())
  }
}

impl Shli for M2<i64> {
#[inline(always)]
  fn shli(self, s: i32) -> Self {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_q(self.unwrap(), s))
    }
  }
}

impl Shri for M2<i64> {
#[inline(always)]
  fn shri(self, s: i32) -> Self {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_q(self.unwrap(), s))
    }
  }
}
