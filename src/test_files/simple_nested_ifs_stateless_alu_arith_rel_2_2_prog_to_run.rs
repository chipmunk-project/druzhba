use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;use druzhba::phv::Phv;
use std::collections::HashMap;
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_0 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_0 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_1 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_1 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_1 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_2 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_2 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_5(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_3 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_3 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_1 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_6(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_4 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_7(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_5 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_2 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_4 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_8(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_9(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_6 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_5 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_2 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_10(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_7 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_11(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_8 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_6 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_3 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_12(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_9 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_13(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_10 (constant : i32) -> i32 {
  constant
}
pub fn init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        if simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_0(simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_0(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_0_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_0_global"])-simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_1_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_0(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_0_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_0_global"]) != 0{
        if simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_1(simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_1(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_1_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_2_global"])-simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_3(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_3_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_1(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_1_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_1_global"]) != 0{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_2(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_2_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_0 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_2(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_2_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_4_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_3(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_3_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_5_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_0_global"]);
        }
        else{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_3(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_3_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_1 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_4(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_4_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_6_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_5(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_5_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_7_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_1_global"]);
        }

        }
        else{
        if simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_2(simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_4(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_4_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_8_global"])-simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_9_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_6(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_6_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_rel_op_2_global"]) != 0{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_5(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_5_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_2 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_7(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_7_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_10_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_8(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_8_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_11_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_2_global"]);
        }
        else{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_6(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Opt_6_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_3 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_9(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_9_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_12_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_10(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_const_10_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_Mux3_13_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0_arith_op_3_global"]);
        }

        }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==0 {
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==2{
        (vec![(phv_containers[0].get_value()+hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==4{
        (vec![(phv_containers[0].get_value()-hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==5{
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"]-phv_containers[0].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==6{
        (vec![((phv_containers[0].get_value()!=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==9{
        (vec![((phv_containers[0].get_value()==hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
        }
        else{
        (vec![((phv_containers[0].get_value()<hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_immediate"])) as i32], Vec::new())
        }

    };
   Box::new(alu)
}
pub fn init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==0 {
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==2{
        (vec![(phv_containers[0].get_value()+hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==4{
        (vec![(phv_containers[0].get_value()-hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==5{
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"]-phv_containers[0].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==6{
        (vec![((phv_containers[0].get_value()!=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==9{
        (vec![((phv_containers[0].get_value()==hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
        }
        else{
        (vec![((phv_containers[0].get_value()<hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_immediate"])) as i32], Vec::new())
        }

    };
   Box::new(alu)
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_0 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_0 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_1 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_1 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_1 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_2 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_2 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_5(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_3 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_3 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_1 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_6(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_4 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_7(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_5 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_2 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_4 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_8(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_9(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_6 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_5 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_2 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_10(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_7 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_11(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_8 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_6 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_3 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    op1 + op2
  }
  else {
  op1 - op2
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_12(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_9 (constant : i32) -> i32 {
  constant
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_13(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_10 (constant : i32) -> i32 {
  constant
}
pub fn init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        if simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_0(simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_0(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_0_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_0_global"])-simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_1_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_0(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_0_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_0_global"]) != 0{
        if simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_1(simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_1(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_1_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_2_global"])-simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_3(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_3_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_1(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_1_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_1_global"]) != 0{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_2(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_2_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_0 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_2(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_2_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_4_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_3(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_3_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_5_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_0_global"]);
        }
        else{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_3(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_3_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_1 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_4(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_4_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_6_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_5(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_5_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_7_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_1_global"]);
        }

        }
        else{
        if simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_2(simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_4(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_4_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_8_global"])-simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), 0, hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_9_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_6(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_6_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_rel_op_2_global"]) != 0{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_5(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_5_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_2 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_7(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_7_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_10_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_8(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_8_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_11_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_2_global"]);
        }
        else{
        state_vec[0] = simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_6(state_vec[0], hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Opt_6_global"])+simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_3 (simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_9(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_9_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_12_global"]), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_10(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_const_10_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_Mux3_13_global"]), hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0_arith_op_3_global"]);
        }

        }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==0 {
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==2{
        (vec![(phv_containers[0].get_value()+hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==4{
        (vec![(phv_containers[0].get_value()-hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==5{
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"]-phv_containers[0].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==6{
        (vec![((phv_containers[0].get_value()!=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==9{
        (vec![((phv_containers[0].get_value()==hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
        }
        else{
        (vec![((phv_containers[0].get_value()<hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_immediate"])) as i32], Vec::new())
        }

    };
   Box::new(alu)
}
pub fn init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==0 {
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==2{
        (vec![(phv_containers[0].get_value()+hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==4{
        (vec![(phv_containers[0].get_value()-hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"]) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==5{
        (vec![(hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"]-phv_containers[0].get_value()) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==6{
        (vec![((phv_containers[0].get_value()!=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==9{
        (vec![((phv_containers[0].get_value()==hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"])) as i32], Vec::new())
        }
        else if hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
        }
        else{
        (vec![((phv_containers[0].get_value()<hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_immediate"])) as i32], Vec::new())
        }

    };
   Box::new(alu)
}
pub fn init_pipeline (hole_vars : HashMap <String, i32>) -> Pipeline { 
  let mut pipeline_stages : Vec<PipelineStage> = Vec::new();

  // Stage 0 stateful ALUs
  let mut stateful_alus_0 : Vec <ALU> = Vec::new();
  let mut stateless_alus_0 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_operand_mux_0_0_0_ctrl"] });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_operand_mux_0_0_1_ctrl"] });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_0_0 : ALU = ALU {sequential_function : init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_0_0(hole_vars.clone()), state_variables : state_variables_0_0, input_muxes : stateful_input_muxes_0_0, output_mux : stateful_output_mux_0_0, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_0);

  // Stage 0 stateless ALUs
  let mut stateless_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_mux1_ctrl"] });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0_mux2_ctrl"] });
  let stateless_output_mux_0_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_output_mux_phv_0_0_ctrl"]};
  let stateless_alu_0_0 : ALU = ALU {sequential_function : init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_0(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_0, output_mux : stateless_output_mux_0_0, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_0);
  let mut stateless_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_mux1_ctrl"] });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1_mux2_ctrl"] });
  let stateless_output_mux_0_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_output_mux_phv_0_1_ctrl"]};
  let stateless_alu_0_1 : ALU = ALU {sequential_function : init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_0_1(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_1, output_mux : stateless_output_mux_0_1, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_1);
  let salu_configs_0 : Vec <i32> = vec![hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_salu_config_0_0"]];
  let pipeline_stage_0 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_0, stateless_atoms : stateless_alus_0 , salu_configs : salu_configs_0};
  pipeline_stages.push(pipeline_stage_0);

  // Stage 1 stateful ALUs
  let mut stateful_alus_1 : Vec <ALU> = Vec::new();
  let mut stateless_alus_1 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_operand_mux_1_0_0_ctrl"] });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_operand_mux_1_0_1_ctrl"] });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_1_0 : ALU = ALU {sequential_function : init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateful_alu_1_0(hole_vars.clone()), state_variables : state_variables_1_0, input_muxes : stateful_input_muxes_1_0, output_mux : stateful_output_mux_1_0, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_0);

  // Stage 1 stateless ALUs
  let mut stateless_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_mux1_ctrl"] });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0_mux2_ctrl"] });
  let stateless_output_mux_1_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_output_mux_phv_1_0_ctrl"]};
  let stateless_alu_1_0 : ALU = ALU {sequential_function : init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_0(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_0, output_mux : stateless_output_mux_1_0, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_0);
  let mut stateless_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_mux1_ctrl"] });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1_mux2_ctrl"] });
  let stateless_output_mux_1_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_output_mux_phv_1_1_ctrl"]};
  let stateless_alu_1_1 : ALU = ALU {sequential_function : init_simple_nested_ifs_stateless_alu_arith_rel_2_2_stateless_alu_1_1(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_1, output_mux : stateless_output_mux_1_1, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_1);
  let salu_configs_1 : Vec <i32> = vec![hole_vars["simple_nested_ifs_stateless_alu_arith_rel_2_2_salu_config_1_0"]];
  let pipeline_stage_1 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_1, stateless_atoms : stateless_alus_1 , salu_configs : salu_configs_1};
  pipeline_stages.push(pipeline_stage_1);

  // Initializing Pipeline using all PipelineStages 
  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);
  pipeline
}
