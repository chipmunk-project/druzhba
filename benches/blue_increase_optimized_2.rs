extern crate druzhba;
use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;use druzhba::phv::Phv;
use std::collections::HashMap;
pub fn name() -> String {
  "blue_increase_pair_stateless_alu_arith_4_2".to_string()
}
pub fn pipeline_depth () -> i32 {
  4
}
pub fn pipeline_width () -> i32 {
  2
}
pub fn num_stateful_operands () -> i32 {
  5
}
pub fn num_stateless_operands () -> i32 {
  2
}
pub fn num_state_variables() -> i32 {
  2
}
pub fn num_stateful_alus() -> i32 {
  2
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((state_vec[0]+phv_containers[0].get_value()-0 == 5) as i32) != 0{
      if ((state_vec[1]+0-0 == 2) as i32) != 0{
        state_vec[0] = state_vec[0]+(5 + 4);
        state_vec[1] = state_vec[1]+(2 + phv_containers[1].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(1 + 0);
        state_vec[1] = state_vec[1]+(3 - 3);
      }

      }
      else if ((state_vec[0]+phv_containers[0].get_value()-0 > 2) as i32)!= 0{
      if ((state_vec[0]+0-0 == 2) as i32) != 0{
        state_vec[0] = 0+(0 + 2);
        state_vec[1] = state_vec[1]+(phv_containers[1].get_value() - phv_containers[1].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(0 + 1);
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() - phv_containers[1].get_value());
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((0+phv_containers[0].get_value()-0 < 2) as i32) != 0{
      if ((0+0-phv_containers[1].get_value() == 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(phv_containers[0].get_value() - 1);
        state_vec[1] = state_vec[1]+(0 + 0);
      }
        else{
        state_vec[0] = state_vec[0]+(0 - 1);
        state_vec[1] = state_vec[1]+(phv_containers[1].get_value() - 2);
      }

      }
      else if ((state_vec[1]+0-phv_containers[0].get_value() != 0) as i32)!= 0{
      if ((0+0-phv_containers[1].get_value() < 4) as i32) != 0{
        state_vec[0] = state_vec[0]+(0 - phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(0 + 0);
      }
        else{
        state_vec[0] = state_vec[0]+(4 - phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(2 - 2);
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_0_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(1-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_0_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()-1) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((state_vec[1]+0-phv_containers[1].get_value() == 5) as i32) != 0{
      if ((state_vec[0]+phv_containers[0].get_value()-0 != 4) as i32) != 0{
        state_vec[0] = state_vec[0]+(0 - 0);
        state_vec[1] = state_vec[1]+(5 - phv_containers[0].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(9 + 2);
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() + 1);
      }

      }
      else if ((state_vec[0]+0-0 == 2) as i32)!= 0{
      if ((state_vec[0]+0-0 == 2) as i32) != 0{
        state_vec[0] = state_vec[0]+(3 + 3);
        state_vec[1] = state_vec[1]+(1 + phv_containers[1].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(9 + 2);
        state_vec[1] = state_vec[1]+(0 + 6);
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((0+phv_containers[1].get_value()-phv_containers[1].get_value() == 2) as i32) != 0{
      if ((state_vec[0]+0-0 > 1) as i32) != 0{
        state_vec[0] = state_vec[0]+(1 + phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() - phv_containers[0].get_value());
      }
        else{
        state_vec[0] = 0+(1 + 1);
        state_vec[1] = state_vec[1]+(0 + 0);
      }

      }
      else if ((state_vec[0]+phv_containers[1].get_value()-0 != 0) as i32)!= 0{
      if ((state_vec[0]+phv_containers[0].get_value()-0 > 1) as i32) != 0{
        state_vec[0] = state_vec[0]+(0 + 0);
        state_vec[1] = state_vec[1]+(0 + 0);
      }
        else{
        state_vec[0] = 0+(1 - phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(5 - 5);
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_1_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(4-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_1_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()+9) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((0+phv_containers[1].get_value()-phv_containers[0].get_value() < 5) as i32) != 0{
      if ((state_vec[1]+0-phv_containers[1].get_value() < 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(0 + 0);
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() - phv_containers[0].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(phv_containers[0].get_value() - phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(0 + 0);
      }

      }
      else if ((0+0-0 == 0) as i32)!= 0{
      if ((state_vec[0]+0-phv_containers[0].get_value() > 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(0 + 1);
        state_vec[1] = state_vec[1]+(0 + 0);
      }
        else{
        state_vec[0] = state_vec[0]+(0 + 1);
        state_vec[1] = state_vec[1]+(2 - 2);
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((state_vec[0]+phv_containers[0].get_value()-0 == 3) as i32) != 0{
      if ((state_vec[0]+0-phv_containers[0].get_value() < 2) as i32) != 0{
        state_vec[0] = state_vec[0]+(0 + 0);
        state_vec[1] = state_vec[1]+(0 + 0);
      }
        else{
        state_vec[0] = state_vec[0]+(phv_containers[1].get_value() + 6);
        state_vec[1] = state_vec[1]+(0 + phv_containers[0].get_value());
      }

      }
      else if ((state_vec[0]+phv_containers[1].get_value()-0 < 2) as i32)!= 0{
      if ((0+0-phv_containers[1].get_value() == 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(phv_containers[0].get_value() - 2);
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() - 0);
      }
        else{
        state_vec[0] = state_vec[0]+(phv_containers[1].get_value() - 3);
        state_vec[1] = state_vec[1]+(0 - 9);
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_2_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(9-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_2_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(2-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((state_vec[0]+0-0 < 0) as i32) != 0{
      if ((state_vec[1]+0-0 == 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(phv_containers[0].get_value() + phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(2 + phv_containers[0].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(phv_containers[0].get_value() + phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(2 + 2);
      }

      }
      else if ((state_vec[0]+0-0 != 0) as i32)!= 0{
      if ((0+0-phv_containers[0].get_value() == 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(2 + phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() + phv_containers[1].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(3 + 1);
        state_vec[1] = state_vec[1]+(phv_containers[1].get_value() + 2);
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if ((0+phv_containers[1].get_value()-0 == 4) as i32) != 0{
      if ((state_vec[0]+phv_containers[1].get_value()-0 < 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(phv_containers[1].get_value() + 2);
        state_vec[1] = state_vec[1]+(3 + 2);
      }
        else{
        state_vec[0] = state_vec[0]+(0 - phv_containers[1].get_value());
        state_vec[1] = state_vec[1]+(5 + phv_containers[0].get_value());
      }

      }
      else if ((state_vec[0]+0-phv_containers[0].get_value() == 0) as i32)!= 0{
      if ((0+phv_containers[1].get_value()-0 == 0) as i32) != 0{
        state_vec[0] = state_vec[0]+(6 - phv_containers[0].get_value());
        state_vec[1] = state_vec[1]+(5 - phv_containers[1].get_value());
      }
        else{
        state_vec[0] = state_vec[0]+(1 + 0);
        state_vec[1] = state_vec[1]+(phv_containers[0].get_value() - phv_containers[0].get_value());
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_3_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(6-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_3_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(5-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
//_h not used
pub fn init_pipeline (_h : HashMap <String, i32>) -> Pipeline { 
  let mut pipeline_stages : Vec<PipelineStage> = Vec::new();

  // Stage 0 stateful ALUs
  let mut stateful_alus_0 : Vec <ALU> = Vec::new();
  let mut stateless_alus_0 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_0 : Vec<i32> = vec![0; 2];
  let stateful_alu_0_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0(HashMap::new()), state_variables : state_variables_0_0, input_muxes : stateful_input_muxes_0_0, output_mux : stateful_output_mux_0_0, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_0);  let mut stateful_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_1 : Vec<i32> = vec![0; 2];
  let stateful_alu_0_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1(HashMap::new()), state_variables : state_variables_0_1, input_muxes : stateful_input_muxes_0_1, output_mux : stateful_output_mux_0_1, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_1);

  // Stage 0 stateless ALUs
  let mut stateless_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_0_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 4};
  let stateless_alu_0_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_0_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_0, output_mux : stateless_output_mux_0_0, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_0);
  let mut stateless_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_0_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 6};
  let stateless_alu_0_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_0_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_1, output_mux : stateless_output_mux_0_1, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_1);
  let salu_configs_0 : Vec <i32> = vec![0,0];
  let output_mux_globals_0 : Vec <i32> = vec![0,0];
  let pipeline_stage_0 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_0, stateless_atoms : stateless_alus_0 , salu_configs : salu_configs_0, output_mux_globals : output_mux_globals_0, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_0);

  // Stage 1 stateful ALUs
  let mut stateful_alus_1 : Vec <ALU> = Vec::new();
  let mut stateless_alus_1 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_0 : Vec<i32> = vec![0; 2];
  let stateful_alu_1_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0(HashMap::new()), state_variables : state_variables_1_0, input_muxes : stateful_input_muxes_1_0, output_mux : stateful_output_mux_1_0, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_0);  let mut stateful_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_1 : Vec<i32> = vec![0; 2];
  let stateful_alu_1_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1(HashMap::new()), state_variables : state_variables_1_1, input_muxes : stateful_input_muxes_1_1, output_mux : stateful_output_mux_1_1, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_1);

  // Stage 1 stateless ALUs
  let mut stateless_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_1_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 6};
  let stateless_alu_1_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_1_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_0, output_mux : stateless_output_mux_1_0, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_0);
  let mut stateless_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_1_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 2};
  let stateless_alu_1_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_1_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_1, output_mux : stateless_output_mux_1_1, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_1);
  let salu_configs_1 : Vec <i32> = vec![0,1];
  let output_mux_globals_1 : Vec <i32> = vec![0,1];
  let pipeline_stage_1 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_1, stateless_atoms : stateless_alus_1 , salu_configs : salu_configs_1, output_mux_globals : output_mux_globals_1, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_1);

  // Stage 2 stateful ALUs
  let mut stateful_alus_2 : Vec <ALU> = Vec::new();
  let mut stateless_alus_2 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_0 : Vec<i32> = vec![0; 2];
  let stateful_alu_2_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0(HashMap::new()), state_variables : state_variables_2_0, input_muxes : stateful_input_muxes_2_0, output_mux : stateful_output_mux_2_0, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_0);  let mut stateful_input_muxes_2_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateful_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateful_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_1 : Vec<i32> = vec![0; 2];
  let stateful_alu_2_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1(HashMap::new()), state_variables : state_variables_2_1, input_muxes : stateful_input_muxes_2_1, output_mux : stateful_output_mux_2_1, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_1);

  // Stage 2 stateless ALUs
  let mut stateless_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_2_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 4};
  let stateless_alu_2_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_2_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_0, output_mux : stateless_output_mux_2_0, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_0);
  let mut stateless_input_muxes_2_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_2_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 4};
  let stateless_alu_2_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_2_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_1, output_mux : stateless_output_mux_2_1, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_1);
  let salu_configs_2 : Vec <i32> = vec![1,0];
  let output_mux_globals_2 : Vec <i32> = vec![1,1];
  let pipeline_stage_2 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_2, stateless_atoms : stateless_alus_2 , salu_configs : salu_configs_2, output_mux_globals : output_mux_globals_2, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_2);

  // Stage 3 stateful ALUs
  let mut stateful_alus_3 : Vec <ALU> = Vec::new();
  let mut stateless_alus_3 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_3_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_3_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_3_0 : Vec<i32> = vec![0; 2];
  let stateful_alu_3_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0(HashMap::new()), state_variables : state_variables_3_0, input_muxes : stateful_input_muxes_3_0, output_mux : stateful_output_mux_3_0, is_stateful: true };
  stateful_alus_3.push(stateful_alu_3_0);  let mut stateful_input_muxes_3_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_3_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_3_1 : Vec<i32> = vec![0; 2];
  let stateful_alu_3_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1(HashMap::new()), state_variables : state_variables_3_1, input_muxes : stateful_input_muxes_3_1, output_mux : stateful_output_mux_3_1, is_stateful: true };
  stateful_alus_3.push(stateful_alu_3_1);

  // Stage 3 stateless ALUs
  let mut stateless_input_muxes_3_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_3_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 6};
  let stateless_alu_3_0 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_3_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_0, output_mux : stateless_output_mux_3_0, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_0);
  let mut stateless_input_muxes_3_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_3_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 7};
  let stateless_alu_3_1 : ALU = ALU {sequential_function : init_blue_increase_pair_stateless_alu_arith_4_2_stateless_alu_3_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_1, output_mux : stateless_output_mux_3_1, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_1);
  let salu_configs_3 : Vec <i32> = vec![0,0];
  let output_mux_globals_3 : Vec <i32> = vec![0,0];
  let pipeline_stage_3 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_3, stateless_atoms : stateless_alus_3 , salu_configs : salu_configs_3, output_mux_globals : output_mux_globals_3, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_3);

  // Initializing Pipeline using all PipelineStages 
  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);
  pipeline
}
