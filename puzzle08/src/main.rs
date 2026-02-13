// use ::std::num::TryFromIntError;

fn double_it(
  n: u64,
  _: i32,
) -> u64 {
  n * 2
}

fn main() {
  // Note that this is a signed integer.
  // Try changing one from 1 to -1 to see what happens.
  let one: i32 = 1;

  let n = double_it(one as _, 3);

  println!("{}", n);

  // let n = double_it(one.into(), 3);

  // println!("{}", n);

  let one_converted_result = one.try_into();

  if let Ok(one_converted) = one_converted_result {
    let n: u64 = double_it(one_converted, 3);

    println!("{}", n);
  }
}
