extern crate simdty;
extern crate llvmint;

use super::super::super::*;

impl Multi for M4<i32> {
  type Elem = i32;
  type Repr = simdty::i32x4;
#[inline(always)]
  fn wrap(s: Self::Repr) -> Self {
    Twice(Twice(s.0, s.1), Twice(s.2, s.3))
  }
#[inline(always)]
  fn unwrap(self) -> Self::Repr {
    simdty::i32x4((self.0).0, (self.0).1, (self.1).0, (self.1).1)
  }
}

impl Set1 for M4<i32> {
#[inline(always)]
  fn set1(a: Self::Elem) -> Self {
    Twice(Twice(a, a), Twice(a, a))
  }
}

impl Add for M4<i32> {
#[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Multi::wrap(self.unwrap() + rhs.unwrap())
  }
}

impl Shli for M4<i32> {
#[inline(always)]
  fn shli(self, s: i32) -> Self {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_d(self.unwrap(), s))
    }
  }
}

impl Shri for M4<i32> {
#[inline(always)]
  fn shri(self, s: i32) -> Self {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_d(self.unwrap(), s))
    }
  }
}

impl Mullo for M4<i32> {
#[inline(always)]
  fn mullo(self, rhs: Self) -> Self::ElemTwice {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pmulu_dq(self.unwrap(), rhs.unwrap()))
    }
  }
}
