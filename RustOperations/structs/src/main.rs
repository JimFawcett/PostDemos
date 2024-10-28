// demo for methods
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
pub struct S<T: Debug + Default + Clone> {
  item: T,
}
impl<T: Debug + Default + Clone> S<T> {
  pub fn new() -> Self {
    Self {
      item: Default::default(),
    }
  }
  pub fn get_item(&mut self) -> &mut T {
    &mut self.item
  }
}
fn main() {
  let mut s = S::<f64>::new();
  println!("{:?}", s);
  let item = s.get_item();
  *item = 3.1415927;
  println!("item = {:?}", item);
  println!("{:?}", s);
}
