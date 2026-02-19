fn main() {
  let b = Box::new([0u32; 10_000_000]);

  println!("{}", b.len());

  let v = vec![0u32; 10_000_000];

  println!("{}", v.len());
}
