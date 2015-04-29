extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSSE3;

impl Abs<M16<i8>> for SSSE3 {
#[inline(always)]
  fn abs(&self, a: M16<i8>) -> M16<i8> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_pabs_b_128(a.unwrap()))
    }
  }
}

impl Sign<M16<i8>> for SSSE3 {
#[inline(always)]
  fn sign(&self, a: M16<i8>, b: M16<i8>) -> M16<i8> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_psign_b_128(a.unwrap(), b.unwrap()))
    }
  }
}
