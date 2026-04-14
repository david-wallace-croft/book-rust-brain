fn main() {
  if 'X' == 'Χ' {
    println!("It matches!");
  } else {
    println!("It does not match.");
  }

  // The Greek capital letter Chi as a Unicode escape
  println!("\u{03A7}");
}
