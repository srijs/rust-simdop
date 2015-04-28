extern crate simdop;

use simdop::*;

fn main() {
  let m1: M2<i64> = set1(1);
  let m2: M2<i64> = set1(2);
  println!("Sum:    {:?}", m1.add(m2));
  println!("Double: {:?}", m1.shli(1));
  println!("Half:   {:?}", m2.shri(1));
  let m3: M4<i32> = set1(1);
  let m4: M4<i32> = set1(2);
  println!("Sum:    {:?}", m3.add(m4));
  println!("Double: {:?}", m3.shli(1));
  println!("Half:   {:?}", m4.shri(1));
}
