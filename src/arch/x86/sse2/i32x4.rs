extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSE2;

impl CmpEq<M4<i32>> for SSE2 {
#[inline(always)]
  fn cmpeq(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    Multi::wrap(a.unwrap() == b.unwrap())
  }
}

impl CmpGt<M4<i32>> for SSE2 {
#[inline(always)]
  fn cmpgt(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    Multi::wrap(a.unwrap() > b.unwrap())
  }
}

impl CmpLt<M4<i32>> for SSE2 {
#[inline(always)]
  fn cmplt(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    Multi::wrap(a.unwrap() < b.unwrap())
  }
}

impl Set1<M4<i32>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i32) -> M4<i32> {
    Twice{
      lo: Twice{lo: a, hi: a},
      hi: Twice{lo: a, hi: a}
    }
  }
}

impl Add<M4<i32>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl Shli<M4<i32>> for SSE2 {
#[inline(always)]
  fn shli(&self, a: M4<i32>, i: i32) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_d(a.unwrap(), i))
    }
  }
}

impl Shri<M4<i32>> for SSE2 {
#[inline(always)]
  fn shri(&self, a: M4<i32>, i: i32) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_d(a.unwrap(), i))
    }
  }
}

impl PackS<M4<i32>> for SSE2 {
#[inline(always)]
  fn packs(&self, a: M4<i32>, b: M4<i32>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_packssdw_128(a.unwrap(), b.unwrap()))
    }
  }
}
