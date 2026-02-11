use ::std::f32::consts::PI;

pub struct Degrees(pub f32);

impl Degrees {
  pub fn new(angle: f32) -> Self {
    Self(angle)
  }
}

pub struct Radians(pub f32);

impl From<Degrees> for Radians {
  fn from(value: Degrees) -> Self {
    Self(value.0 * PI / 180.0)
  }
}

fn main() {
  let one_eighty_degrees: Degrees = Degrees::new(180.0);

  let one_eighty_radians: Radians = one_eighty_degrees.into();

  println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}
