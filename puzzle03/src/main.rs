#![warn(clippy::pedantic)]

fn main() {
  let x: u64 = 4_294_967_296;

  let y: u32 = x as u32;

  let z: u64 = y as u64;

  if x == z {
    println!("x equals z");
  } else {
    println!("x does not equal z: x = {x}, y = {y}, z = {z}");
  }

  println!("::std::u32::MAX = {}", ::std::u32::MAX);
}
