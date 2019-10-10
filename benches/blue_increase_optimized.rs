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
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_0() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_1() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_2() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_3() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_4() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_5() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_6() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_7() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_8() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_9() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 > op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_10() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_11() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_4(op : i32) -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_12() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_13() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_14() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_15() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_16() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_17() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_18() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_19() -> i32{
  0
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_0_const_19()));
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_0() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_1() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_2() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_3() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_4() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_5() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_6() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_7() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_8() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_9() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 != op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_10() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_11() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_12() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_13() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_14() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_15() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_16() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_17() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_18() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_19() -> i32{
  2
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_0_1_const_19()));
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
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_0() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 != op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_1() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_2() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_3() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_4() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_5() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_6() -> i32{
  9
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_7() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_8() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_9() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_10() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_11() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_12() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_13() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_14() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_15() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_16() -> i32{
  9
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_17() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_18() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_19() -> i32{
  6
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_0_const_19()));
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_0() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 > op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_1() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_2() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_3() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_4() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_5() -> i32{
  9
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_2(op : i32) -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_6() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_7() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_8() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_9() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 != op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_10() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 > op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_11() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_12() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_13() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_14() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_15() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_6(op : i32) -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_16() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_17() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_18() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_19() -> i32{
  5
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_1_1_const_19()));
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
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_0() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_1() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_2() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_3() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_4() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_5() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_6() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_7() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_8() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_9() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_10() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 > op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_11() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_12() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_13() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_14() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_15() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_16() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_17() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_18() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_19() -> i32{
  2
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_0_const_19()));
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_0() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_1() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_2() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_3() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_4() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_5() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_6() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_7() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_8() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_9() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_10() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_11() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_12() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_13() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_14() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_15() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_16() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_17() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_18() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_19() -> i32{
  9
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_2_1_const_19()));
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
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_0() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_1() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_2() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_3() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_4() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_5() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_6() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_7() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_8() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_9() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 != op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_10() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_11() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_12() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_13() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_14() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_15() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_16() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_17() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_18() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_19() -> i32{
  2
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_0_const_19()));
      }

      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_0() -> i32{
  4
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_1(op1 : i32, op2 : i32) -> i32{
  (op1 < op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_5(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_1() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_0(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_6(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_2() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_7(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_3() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_1(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_8(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_4() -> i32{
  3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_9(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_5() -> i32{
  2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_2(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_2(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_10(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_6() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_11(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_7() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_3(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_3(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_12(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_8() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_13(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_9() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_2(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_14(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_15(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_16(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_10() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_3(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_17(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_18(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_19(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_11() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_4(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_4(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_20(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_12() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_21(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_13() -> i32{
  6
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_5(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_5(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_22(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_14() -> i32{
  5
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_23(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_15() -> i32{
  9
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_6(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_6(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_24(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_16() -> i32{
  1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_25(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_17() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_7(op : i32) -> i32{
  op
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_7(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_26(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_18() -> i32{
  0
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_27(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_19() -> i32{
  0
}
pub fn init_blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_0(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_0()) != 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_3(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_5(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_1()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_0(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_0(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_6(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_2()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_7(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_3()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_1(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_1(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_8(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_4()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_9(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_5()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_2(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_10(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_6()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_11(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_7()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_3(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_12(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_8()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_13(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_9()));
      }

      }
      else if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_2(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_14(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_15(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_16(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_10())!= 0{
      if blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_3(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_17(state_vec[0], state_vec[1], 0)+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_18(phv_containers[0].get_value(), phv_containers[1].get_value(), 0)-blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_19(phv_containers[0].get_value(), phv_containers[1].get_value(), 0), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_11()) != 0{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_4(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_4(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_20(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_12()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_21(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_13()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_5(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_5(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_22(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_14()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_23(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_15()));
      }
        else{
        state_vec[0] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_6(state_vec[0])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_6(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_24(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_16()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_25(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_17()));
        state_vec[1] = blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_7(state_vec[1])+blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_7(blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_26(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_18()), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_27(phv_containers[0].get_value(), phv_containers[1].get_value(), blue_increase_pair_stateless_alu_arith_4_2_stateful_alu_3_1_const_19()));
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
  let pipeline_stage_0 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_0, stateless_atoms : stateless_alus_0 , salu_configs : salu_configs_0, output_mux_globals : output_mux_globals_0 };
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
  let pipeline_stage_1 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_1, stateless_atoms : stateless_alus_1 , salu_configs : salu_configs_1, output_mux_globals : output_mux_globals_1 };
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
  let pipeline_stage_2 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_2, stateless_atoms : stateless_alus_2 , salu_configs : salu_configs_2, output_mux_globals : output_mux_globals_2 };
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
  let pipeline_stage_3 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_3, stateless_atoms : stateless_alus_3 , salu_configs : salu_configs_3, output_mux_globals : output_mux_globals_3 };
  pipeline_stages.push(pipeline_stage_3);

  // Initializing Pipeline using all PipelineStages 
  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);
  pipeline
}
