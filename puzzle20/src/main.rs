fn main() {
  #[expect(unused_variables)]
  let hello = || println!("Hello, World!");

  let hello = || println!("Bonjour le monde");

  hello();
}

#[expect(dead_code)]
enum Language {
  English,
  French,
}

// This static dispatch const fn executes at compile-time
#[expect(dead_code)]
const fn hello_const(language: Language) -> &'static str {
  match language {
    Language::English => "Hello, World!",
    Language::French => "Bonjour le monde",
  }
}

// This dynamic dispatch fn executes at run-time
#[expect(dead_code)]
fn hello_dynamic(language: Language) {
  match language {
    Language::English => || println!("Hello, World!"),
    Language::French => || println!("Bonjour le monde"),
  };
}
