extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSSE3;

impl Abs<M4<i32>> for SSSE3 {
#[inline(always)]
  fn abs(&self, a: M4<i32>) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_pabs_d_128(a.unwrap()))
    }
  }
}

impl HAdd<M4<i32>> for SSSE3 {
#[inline(always)]
  fn hadd(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_phadd_d_128(a.unwrap(), b.unwrap()))
    }
  }
}

impl HSub<M4<i32>> for SSSE3 {
#[inline(always)]
  fn hsub(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_phsub_d_128(a.unwrap(), b.unwrap()))
    }
  }
}

impl Sign<M4<i32>> for SSSE3 {
#[inline(always)]
  fn sign(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_psign_d_128(a.unwrap(), b.unwrap()))
    }
  }
}
