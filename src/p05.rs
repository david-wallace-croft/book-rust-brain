pub fn puzzle5() {
  const HELLO_WORLD: &str = "Halló heimur!";

  println!("{HELLO_WORLD} is {} characters long.", HELLO_WORLD.len());

  println!(
    "{HELLO_WORLD} is {} characters long.",
    HELLO_WORLD.chars().count()
  );
}
