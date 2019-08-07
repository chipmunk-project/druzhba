
#[derive(Clone)]
pub struct PhvContainer <T> {
  /*PHV Containers do not have names, just indexes in the PHV*/
  pub field_value: T,
}

impl<T> PhvContainer<T> where T : Clone {
  pub fn new (value: T) -> Self{
      PhvContainer{ field_value: value, }
  }
  pub fn get_value(self) -> T {
      self.field_value.clone()
  }
}
