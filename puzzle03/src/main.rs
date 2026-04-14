#![warn(clippy::pedantic)]

fn main() {
  let x: u64 = 4_294_967_296;

  #[expect(clippy::cast_possible_truncation)]
  let y: u32 = x as u32;

  let z: u64 = u64::from(y);

  if x == z {
    println!("x equals z");
  } else {
    println!("x does not equal z: x = {x}, y = {y}, z = {z}");
  }

  println!("u32::MAX = {}", u32::MAX);
}
