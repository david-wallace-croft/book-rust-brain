fn main() {
  let mut my_vec = Vec::with_capacity(1);

  my_vec.push("1");

  println!("{}", my_vec.capacity());

  my_vec.push("2");

  println!("{}", my_vec.capacity());

  // for s in vec![
  //   "3", "4", "5", "6", "7",
  // ] {
  //   my_vec.push(s);
  // }

  // println!("{}", my_vec.capacity());

  // my_vec.extend(vec![
  //   "3", "4", "5", "6", "7",
  // ]);

  // println!("{}", my_vec.capacity());
}
