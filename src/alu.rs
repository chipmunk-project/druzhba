
use crate::phv_container::PhvContainer;
use crate::phv::Phv;


#[derive(Clone)]
pub struct ALU{

    // Function that ALU will use against incoming Phvs 
    sequential_function :
        fn (&mut Phv, 
            &mut PhvContainer <i32>, 
            &mut PhvContainer <Vec <i32> >),

    // state_scalar and state_array capture the internal
    // state of the atom that map String fields to i32
    // and Vec <i32> respectively
    state_scalar : PhvContainer <i32>,
    state_array : PhvContainer <Vec <i32> >,
        
}

impl ALU {
       
    pub fn new (function : fn (&mut Phv, 
                               &mut PhvContainer <i32>, 
                               &mut PhvContainer <Vec <i32> >),
                new_state_scalar : PhvContainer <i32>,
                new_state_array : PhvContainer <Vec <i32> >) 
                -> ALU {

      ALU { sequential_function : function,
            state_scalar : new_state_scalar, 
            state_array : new_state_array }
    }

    // Receives mutable reference to Phv and calls underlying 
    // function, sequential_function using state_scalar and 
    // state_array. Mutates Phv in place with appropriate 
    // packet values.
    pub fn run (&mut self, packet : &mut Phv) {
      
      assert! (!packet.is_bubble() );
      (self.sequential_function) 
          (packet, 
           &mut self.state_scalar, 
           &mut self.state_array );
    }
   
}



