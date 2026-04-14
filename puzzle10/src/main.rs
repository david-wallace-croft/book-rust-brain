fn main() {
  if 'X' == 'Χ' {
    println!("It matches!");
  } else {
    println!("It does not match.");
  }

  // The capital letter X as a Unicode escape
  println!("\u{0058}");

  // The Greek capital letter Chi as a Unicode escape
  println!("\u{03A7}");

  if 'X' == '\u{58}' {
    println!("It matches!");
  } else {
    println!("It does not match.");
  }
}
