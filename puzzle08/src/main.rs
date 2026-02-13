// use ::std::num::TryFromIntError;

fn double_it(
  n: u64,
  _: i32,
) -> u64 {
  n * 2
}

fn main() {
  let one: i32 = 1;

  let n: u64 = double_it(one as _, 3);

  println!("{}", n);

  // let n: u64 = double_it(one.into(), 3);

  // println!("{}", n);

  let one_converted_result = one.try_into();

  if let Ok(one_converted) = one_converted_result {
    let n: u64 = double_it(one_converted, 3);

    println!("{}", n);
  }
}
