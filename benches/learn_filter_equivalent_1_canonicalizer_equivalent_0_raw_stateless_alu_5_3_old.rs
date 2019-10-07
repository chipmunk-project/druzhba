extern crate druzhba;
use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;use druzhba::phv::Phv;
use std::collections::HashMap;
pub fn name() -> String {
  "learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string()
}
pub fn pipeline_depth () -> i32 {
  5
}
pub fn pipeline_width () -> i32 {
  3
}
pub fn num_stateful_operands () -> i32 {
  1
}
pub fn num_stateless_operands () -> i32 {
  3
}
pub fn num_state_variables() -> i32 {
  1
}
pub fn num_stateful_alus() -> i32 {
  3
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op1
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_0_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_0_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![((phv_containers[0].get_value()>=0)) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_0_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())

    };
   Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op1
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_1_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_1_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }


    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_1_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())

    };
   Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0_Opt_0(op : i32) -> i32{
  op
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0_const_0() -> i32{
  2
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2_Opt_0(op : i32) -> i32{
  op
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op1
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2_const_0() -> i32{
  2
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_2_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_2_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }


    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_2_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())

    };
   Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0_Opt_0(op : i32) -> i32{
  op
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0_const_0() -> i32{
  3
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1_const_0() -> i32{
  3
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2_Opt_0(op : i32) -> i32{
  op
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2_const_0() -> i32{
  2
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_3_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_3_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_3_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }


    };
   Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0_Opt_0(op : i32) -> i32{
  op
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0_const_0() -> i32{
  3
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op2
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2_Opt_0(op : i32) -> i32{
  0
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2_Mux2_0(op1 : i32, op2 : i32) -> i32{
  op1
}
fn learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2_const_0() -> i32{
  1
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
        state_vec[0] = learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2_Opt_0(state_vec[0])+learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2_Mux2_0(phv_containers[0].get_value(), learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2_const_0());
    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_4_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_4_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(1) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_4_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(2) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_pipeline (h : HashMap <String, i32>) -> Pipeline { 
  let mut pipeline_stages : Vec<PipelineStage> = Vec::new();

  // Stage 0 stateful ALUs
  let mut stateful_alus_0 : Vec <ALU> = Vec::new();
  let mut stateless_alus_0 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_0_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_0(HashMap::new()), state_variables : state_variables_0_0, input_muxes : stateful_input_muxes_0_0, output_mux : stateful_output_mux_0_0, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_0);  let mut stateful_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_1 : Vec<i32> = vec![0; 1];
  let stateful_alu_0_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_1(HashMap::new()), state_variables : state_variables_0_1, input_muxes : stateful_input_muxes_0_1, output_mux : stateful_output_mux_0_1, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_1);  let mut stateful_input_muxes_0_2 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_2 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_2 : Vec<i32> = vec![0; 1];
  let stateful_alu_0_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_0_2(HashMap::new()), state_variables : state_variables_0_2, input_muxes : stateful_input_muxes_0_2, output_mux : stateful_output_mux_0_2, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_2);

  // Stage 0 stateless ALUs
  let mut stateless_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_0_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 2};
  let stateless_alu_0_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_0_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_0, output_mux : stateless_output_mux_0_0, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_0);
  let mut stateless_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_0_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 3};
  let stateless_alu_0_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_0_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_1, output_mux : stateless_output_mux_0_1, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_1);
  let mut stateless_input_muxes_0_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_0_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 0};
  let stateless_alu_0_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_0_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_2, output_mux : stateless_output_mux_0_2, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_2);
  let salu_configs_0 : Vec <i32> = vec![0,0,1];
  let output_mux_globals_0 : Vec <i32> = vec![0,1,1];
  let pipeline_stage_0 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_0, stateless_atoms : stateless_alus_0 , salu_configs : salu_configs_0, output_mux_globals : output_mux_globals_0 };
  pipeline_stages.push(pipeline_stage_0);

  // Stage 1 stateful ALUs
  let mut stateful_alus_1 : Vec <ALU> = Vec::new();
  let mut stateless_alus_1 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_1_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_0(HashMap::new()), state_variables : state_variables_1_0, input_muxes : stateful_input_muxes_1_0, output_mux : stateful_output_mux_1_0, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_0);  let mut stateful_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_1 : Vec<i32> = vec![0; 1];
  let stateful_alu_1_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_1(HashMap::new()), state_variables : state_variables_1_1, input_muxes : stateful_input_muxes_1_1, output_mux : stateful_output_mux_1_1, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_1);  let mut stateful_input_muxes_1_2 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_2 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_2 : Vec<i32> = vec![0; 1];
  let stateful_alu_1_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_1_2(HashMap::new()), state_variables : state_variables_1_2, input_muxes : stateful_input_muxes_1_2, output_mux : stateful_output_mux_1_2, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_2);

  // Stage 1 stateless ALUs
  let mut stateless_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_1_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_1_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_1_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_0, output_mux : stateless_output_mux_1_0, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_0);
  let mut stateless_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  let stateless_output_mux_1_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 3};
  let stateless_alu_1_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_1_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_1, output_mux : stateless_output_mux_1_1, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_1);
  let mut stateless_input_muxes_1_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_1_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 0};
  let stateless_alu_1_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_1_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_2, output_mux : stateless_output_mux_1_2, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_2);
  let salu_configs_1 : Vec <i32> = vec![1,0,0];
  let output_mux_globals_1 : Vec <i32> = vec![1,0,1];
  let pipeline_stage_1 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_1, stateless_atoms : stateless_alus_1 , salu_configs : salu_configs_1, output_mux_globals : output_mux_globals_1 };
  pipeline_stages.push(pipeline_stage_1);

  // Stage 2 stateful ALUs
  let mut stateful_alus_2 : Vec <ALU> = Vec::new();
  let mut stateless_alus_2 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_2_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_0(HashMap::new()), state_variables : state_variables_2_0, input_muxes : stateful_input_muxes_2_0, output_mux : stateful_output_mux_2_0, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_0);  let mut stateful_input_muxes_2_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_1 : Vec<i32> = vec![0; 1];
  let stateful_alu_2_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_1(HashMap::new()), state_variables : state_variables_2_1, input_muxes : stateful_input_muxes_2_1, output_mux : stateful_output_mux_2_1, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_1);  let mut stateful_input_muxes_2_2 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_2 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_2 : Vec<i32> = vec![0; 1];
  let stateful_alu_2_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_2_2(HashMap::new()), state_variables : state_variables_2_2, input_muxes : stateful_input_muxes_2_2, output_mux : stateful_output_mux_2_2, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_2);

  // Stage 2 stateless ALUs
  let mut stateless_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  let stateless_output_mux_2_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_2_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_2_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_0, output_mux : stateless_output_mux_2_0, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_0);
  let mut stateless_input_muxes_2_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_2_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 3};
  let stateless_alu_2_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_2_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_1, output_mux : stateless_output_mux_2_1, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_1);
  let mut stateless_input_muxes_2_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  let stateless_output_mux_2_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 2};
  let stateless_alu_2_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_2_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_2, output_mux : stateless_output_mux_2_2, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_2);
  let salu_configs_2 : Vec <i32> = vec![0,1,0];
  let output_mux_globals_2 : Vec <i32> = vec![0,1,0];
  let pipeline_stage_2 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_2, stateless_atoms : stateless_alus_2 , salu_configs : salu_configs_2, output_mux_globals : output_mux_globals_2 };
  pipeline_stages.push(pipeline_stage_2);

  // Stage 3 stateful ALUs
  let mut stateful_alus_3 : Vec <ALU> = Vec::new();
  let mut stateless_alus_3 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_3_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_3_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_3_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_3_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_0(HashMap::new()), state_variables : state_variables_3_0, input_muxes : stateful_input_muxes_3_0, output_mux : stateful_output_mux_3_0, is_stateful: true };
  stateful_alus_3.push(stateful_alu_3_0);  let mut stateful_input_muxes_3_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_3_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_3_1 : Vec<i32> = vec![0; 1];
  let stateful_alu_3_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_1(HashMap::new()), state_variables : state_variables_3_1, input_muxes : stateful_input_muxes_3_1, output_mux : stateful_output_mux_3_1, is_stateful: true };
  stateful_alus_3.push(stateful_alu_3_1);  let mut stateful_input_muxes_3_2 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_3_2 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_3_2 : Vec<i32> = vec![0; 1];
  let stateful_alu_3_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_3_2(HashMap::new()), state_variables : state_variables_3_2, input_muxes : stateful_input_muxes_3_2, output_mux : stateful_output_mux_3_2, is_stateful: true };
  stateful_alus_3.push(stateful_alu_3_2);

  // Stage 3 stateless ALUs
  let mut stateless_input_muxes_3_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_3_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_3_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_3_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_0, output_mux : stateless_output_mux_3_0, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_0);
  let mut stateless_input_muxes_3_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_3_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 2};
  let stateless_alu_3_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_3_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_1, output_mux : stateless_output_mux_3_1, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_1);
  let mut stateless_input_muxes_3_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  let stateless_output_mux_3_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 3};
  let stateless_alu_3_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_3_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_2, output_mux : stateless_output_mux_3_2, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_2);
  let salu_configs_3 : Vec <i32> = vec![0,0,0];
  let output_mux_globals_3 : Vec <i32> = vec![0,0,0];
  let pipeline_stage_3 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_3, stateless_atoms : stateless_alus_3 , salu_configs : salu_configs_3, output_mux_globals : output_mux_globals_3 };
  pipeline_stages.push(pipeline_stage_3);

  // Stage 4 stateful ALUs
  let mut stateful_alus_4 : Vec <ALU> = Vec::new();
  let mut stateless_alus_4 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_4_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_4_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_4_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_4_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_4_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_0(HashMap::new()), state_variables : state_variables_4_0, input_muxes : stateful_input_muxes_4_0, output_mux : stateful_output_mux_4_0, is_stateful: true };
  stateful_alus_4.push(stateful_alu_4_0);  let mut stateful_input_muxes_4_1 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_4_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_4_1 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_4_1 : Vec<i32> = vec![0; 1];
  let stateful_alu_4_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_1(HashMap::new()), state_variables : state_variables_4_1, input_muxes : stateful_input_muxes_4_1, output_mux : stateful_output_mux_4_1, is_stateful: true };
  stateful_alus_4.push(stateful_alu_4_1);  let mut stateful_input_muxes_4_2 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, packets : Vec::new(), state : Vec::new() };
  stateful_input_muxes_4_2.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_4_2 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_4_2 : Vec<i32> = vec![0; 1];
  let stateful_alu_4_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateful_alu_4_2(HashMap::new()), state_variables : state_variables_4_2, input_muxes : stateful_input_muxes_4_2, output_mux : stateful_output_mux_4_2, is_stateful: true };
  stateful_alus_4.push(stateful_alu_4_2);

  // Stage 4 stateless ALUs
  let mut stateless_input_muxes_4_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_4_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_4_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_4_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_4_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 2};
  let stateless_alu_4_0 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_4_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_4_0, output_mux : stateless_output_mux_4_0, is_stateful: false };
  stateless_alus_4.push(stateless_alu_4_0);
  let mut stateless_input_muxes_4_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_4_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_4_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_4_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_4_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 0};
  let stateless_alu_4_1 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_4_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_4_1, output_mux : stateless_output_mux_4_1, is_stateful: false };
  stateless_alus_4.push(stateless_alu_4_1);
  let mut stateless_input_muxes_4_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_4_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_4_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_4_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_4_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 0};
  let stateless_alu_4_2 : ALU = ALU {sequential_function : init_learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_stateless_alu_4_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_4_2, output_mux : stateless_output_mux_4_2, is_stateful: false };
  stateless_alus_4.push(stateless_alu_4_2);
  let salu_configs_4 : Vec <i32> = vec![0,0,0];
  let output_mux_globals_4 : Vec <i32> = vec![0,1,0];
  let pipeline_stage_4 : PipelineStage = PipelineStage {stateful_atoms : stateful_alus_4, stateless_atoms : stateless_alus_4 , salu_configs : salu_configs_4, output_mux_globals : output_mux_globals_4 };
  pipeline_stages.push(pipeline_stage_4);

  // Initializing Pipeline using all PipelineStages 
  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);
  pipeline
}
