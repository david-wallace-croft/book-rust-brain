pub fn puzzle6() {
  if 0.1 + 0.2 == 0.3 {
    println!("Arithmetic still works.");
  } else {
    println!("Please reboot the universe.");
  }

  if (0.1_f64 + 0.2_f64 - 0.3_f64).abs() < std::f64::EPSILON {
    println!("Arithmetic works");
  }
}
