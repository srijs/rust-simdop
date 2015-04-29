extern crate simdop;
extern crate simdty;
extern crate llvmint;

use simdop::*;

fn main() {

  let sse2 = arch::x86::SSE2::detect().unwrap();

  let m1: M2<i64> = sse2.set1(1);
  let m2: M2<i64> = sse2.set1(2);
  println!("Sum:    {:?}", sse2.add(m1, m2));
  println!("Double: {:?}", sse2.shli(m1, 1));
  println!("Half:   {:?}", sse2.shri(m2, 1));
  let m3: M4<i32> = sse2.set1(1);
  let m4: M4<i32> = sse2.set1(2);
  println!("Sum:    {:?}", sse2.add(m3, m4));
  println!("Double: {:?}", sse2.shli(m3, 1));
  println!("Half:   {:?}", sse2.shri(m4, 1));

}
