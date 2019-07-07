  /*By design, PHV Containers are only supposed to hold one field and one value. Hashmaps are intended to hold
  multiple key-value pairs, so I think this way would be better*/
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

    // impl<T> Clone for PhvContainer<T> {
    //     fn clone(&self) -> PhvContainer<T> {
    //         *self
    //     }
    // }



// impl<T> Copy for Generate<T> {}

// impl<T> Clone for Generate<T> {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

