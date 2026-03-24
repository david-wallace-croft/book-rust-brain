fn main() {
  let mut floats: Vec<f64> = vec![
    3.1, 1.2, 4.5, 0.3,
  ];

  // floats.sort();

  floats.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());

  println!("{:#?}", floats);

  let mut floats2: Vec<f64> = vec![
    3.1,
    1.2,
    4.5,
    0.3,
    f64::NAN,
  ];

  floats2.sort_by(|a: &f64, b: &f64| a.total_cmp(b));

  println!("{:#?}", floats2);
}
