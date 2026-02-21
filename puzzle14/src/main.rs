struct VeryImportantMessage {
  _message_type: u8,
  _destination: u16,
}

fn main() {
  println!(
    "VeryImportantMessage occupies {} bytes",
    ::std::mem::size_of::<VeryImportantMessage>()
  );
}
