  use std::clone::Clone;

  #[derive(Clone, Default)]
    pub struct PhvContainer <T> {
      /*PHV Containers do not have names, just indexes in the PHV*/
      pub field_value: T
    }

    impl<T> PhvContainer<T> {
      pub fn new (value: T) -> Self{
          PhvContainer{field_value: value}
      }
      pub fn getValue(self) -> T {
          self.field_value
      }
    }