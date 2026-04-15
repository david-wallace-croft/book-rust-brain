// use std::ops::{RangeFull, RangeTo};

#[expect(clippy::no_effect)]
fn main() {
  .. .. ..;

  // let _: RangeTo<RangeTo<RangeFull>> = .. .. ..;
}
