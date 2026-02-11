use std::num::Wrapping;

fn main() {
  let mut counter: Wrapping<i8> = Wrapping(0i8);

  // if let Some(n) = x.checked_add(b)

  loop {
    println!("{counter}");

    counter += Wrapping(1i8);
  }
}
