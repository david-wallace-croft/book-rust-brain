fn main() {
  #[expect(unused_variables)]
  let hello = || println!("Hello, World!");

  let hello = || println!("Bonjour le monde");

  hello();
}
