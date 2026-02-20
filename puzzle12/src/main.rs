fn main() {
  for _ in 0..1_000 {
    let buffer = (0..1_000).collect::<Vec<u32>>();

    ::std::mem::forget(buffer);

    print!(".");
  }
}
