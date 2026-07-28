fn main() {
  let mut my_vec = Vec::with_capacity(1);

  println!("capacity afore pushing: {}", my_vec.capacity());

  my_vec.push("1");

  println!("capacity after push(1): {}", my_vec.capacity());

  my_vec.push("2");

  println!("capacity after push(2): {}", my_vec.capacity());

  for s in vec![
    "3", "4", "5", "6", "7", "8", "9",
  ] {
    my_vec.push(s);

    println!("capacity after push({s}): {}", my_vec.capacity());
  }

  // my_vec.extend(vec![
  //   "3", "4", "5", "6", "7",
  // ]);

  // println!("{}", my_vec.capacity());
}
