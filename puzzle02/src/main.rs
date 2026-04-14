use std::io::stdin;

fn main() {
  println!("What is 3 + 2? Type your answer and press enter.");

  let mut input: String = String::new();

  #[expect(clippy::read_line_without_trim)]
  stdin()
    .read_line(&mut input)
    .expect("Unable to read standard input");

  if input == "5" {
    println!("Correct!");
  } else {
    println!("Incorrect!");
  }
}
