extern crate druzhba;

use druzhba::output_mux::OutputMux;
use druzhba::input_mux::InputMux;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::alu::ALU;
use druzhba::alu::StateVar;

use prog_to_run;
use test_files::blue_increase_pair_stateless_alu_arith_2_2_prog_to_run;
use test_files::blue_increase_pair_stateless_alu_2_2_prog_to_run;
use test_files::marple_new_flow_pair_stateless_alu_arith_rel_cond_bool_3_3_prog_to_run;
use test_files::marple_new_flow_pair_stateless_alu_arith_rel_cond_2_2_prog_to_run;
use test_files::marple_new_flow_nested_ifs_stateless_alu_arith_rel_cond_2_2_prog_to_run;
use test_files::marple_new_flow_if_else_raw_stateless_alu_2_2_prog_to_run;
use test_files::simple_sub_stateless_alu_arith_rel_2_2_prog_to_run;
use test_files::simple_pred_raw_stateless_alu_arith_rel_cond_bool_2_2_prog_to_run;
use test_files::simple_nested_ifs_stateless_alu_arith_rel_2_2_prog_to_run;
use test_files::simple_pair_stateless_alu_arith_2_2_prog_to_run;
use test_files::simple_if_else_raw_stateless_alu_arith_rel_2_2_prog_to_run;
use test_files::simple_raw_stateless_alu_2_2_prog_to_run;
use test_files::simple_raw_stateless_alu_arith_2_2_prog_to_run;
use test_files::simple_raw_stateless_alu_arith_rel_cond_2_2_prog_to_run;

use rand::Rng;
use std::fs;
use std::collections::HashMap;
use std::process::Command;
/* Test helper functions */
fn init_input_phvs (ticks : i32,
                   num_containers : i32,
                   num_state_vars : i32,
                   num_stateful_alus : i32) -> Vec <Phv <i32> >
{
  let mut input_phvs : Vec <Phv <i32> > = Vec::new();
    // Initializes packet with all of the input fields
    // along with a random value
  for _i in 0..ticks {
    let mut packet : Phv<i32> = Phv::new();
    for _j in (0..num_containers){
        packet.add_container_to_phv(PhvContainer {
            field_value : rand::thread_rng().gen_range(0,100),
      }); 
    }
    let mut state_vec : Vec <Vec <i32>>  = Vec::new();
    for _i in 0..num_stateful_alus {
      let mut tmp_state_vec : Vec <i32> = Vec::new();
      for _j in 0..num_state_vars {
        tmp_state_vec.push (rand::thread_rng().gen_range(0,100));
      }
      state_vec.push (tmp_state_vec);
    }
    packet.set_state (state_vec);
    input_phvs.push (packet.clone());
  }
  input_phvs

}
// Initializes hole config HashMap and initializes pipeline.
// Runs input phvs through pipeline and returns the resulting
// output phvs
fn extract_hole_cfgs (hole_cfgs_file : String) -> HashMap <String, i32 > {
  let mut hole_cfgs_map : HashMap <String, i32> = HashMap::new();
  let hole_cfgs_file_contents : String = fs::read_to_string(hole_cfgs_file).expect ("Error: Hole configs file could not be found");
  let hole_cfgs_file_vec : Vec <String> = hole_cfgs_file_contents
                                          .split ("\n")
                                          .map (|s| s.to_string())
                                          .collect();

  for hole_var in hole_cfgs_file_vec {
      let hole_entry : Vec <&str> = hole_var
                                    .split("=")
                                    .map(|s| s.trim())
                                    .collect();
      if hole_entry.len() < 2 {
        continue;
      }
      hole_cfgs_map.insert (hole_entry[0].to_string(), 
                            hole_entry[1].to_string().parse::<i32>()
                                                     .expect ("Error: hole value set to non-integer value"));
  }
  hole_cfgs_map
}

fn run_pipeline (input_phvs : Vec <Phv <i32> >,
                 mut pipeline : Pipeline,
                 ticks : i32) -> Vec <Phv <i32 > > {
  let mut output_phvs : Vec <Phv <i32> > = Vec::new();
  for t in 0..ticks {
    let new_packet : Phv<i32> = 
        pipeline.tick (input_phvs[t as usize].clone());
    if !new_packet.is_bubble() {
      output_phvs.push(new_packet.clone());
    }
  }
  output_phvs
}


// Maintains the same assertions as the specification
// for the simple.sk spec in Chipmunk
fn test_simple (input_phvs : Vec <Phv <i32> >,
                output_phvs : Vec <Phv <i32> >) {

  for i in 0..output_phvs.len() {
    assert!(output_phvs[i][0].get_value() ==
            input_phvs[i].get_state()[0][0] + 1);

  }
}
fn test_marple_new_flow (input_phvs : Vec <Phv <i32> >,
                         output_phvs : Vec <Phv <i32> >) {

  for i in 0..output_phvs.len() {
    if input_phvs[i].get_state()[0][0] == 0 {
      assert! (output_phvs[i][0].get_value()==1);
      assert! (output_phvs[i].get_state()[0][0]==1);
    }
  }
}
fn test_blue_increase (input_phvs : Vec <Phv <i32> >,
                       output_phvs : Vec <Phv <i32> >) {

  for i in 0..output_phvs.len() {
    assert!(output_phvs[i][1].get_value() == input_phvs[i][0].get_value() - 1);
    if output_phvs[i][1].get_value() > input_phvs[i].get_state()[1][0]{
      assert!(output_phvs[i].get_state()[0][0] == input_phvs[i].get_state()[0][0]+1);
      assert!(output_phvs[i].get_state()[1][0] == input_phvs[i][0].get_value());
    }
  }
}

/* Simple tests */
#[test]
fn test_simple_raw_stateless_alu_2_2 () {

  let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1,1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

  let mut pipeline : Pipeline = 
      simple_raw_stateless_alu_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());

  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);
  test_simple (input_phvs, output_phvs);
}

#[test]
fn test_simple_raw_stateless_alu_arith_2_2 () {

  let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1,1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_raw_stateless_alu_arith_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

  let mut pipeline : Pipeline = 
      simple_raw_stateless_alu_arith_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);
  test_simple (input_phvs, output_phvs);
}
#[test]
fn test_simple_raw_stateless_alu_arith_rel_cond_2_2 () {

  let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_raw_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

  let mut pipeline : Pipeline = 
      simple_raw_stateless_alu_arith_rel_cond_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_simple (input_phvs, output_phvs);
}


#[test]
fn test_simple_if_else_raw_stateless_alu_arith_rel_2_2 () {
   let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_if_else_raw_stateless_alu_arith_rel_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      simple_if_else_raw_stateless_alu_arith_rel_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);
  test_simple (input_phvs, output_phvs);
}

#[test]
fn test_simple_pair_stateless_alu_arith_2_2 () {
   let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 2, 1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_pair_stateless_alu_arith_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      simple_pair_stateless_alu_arith_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_simple (input_phvs, output_phvs);
}
#[test]
fn test_simple_nested_ifs_stateless_alu_arith_rel_2_2 () {
   let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_nested_ifs_stateless_alu_arith_rel_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      simple_nested_ifs_stateless_alu_arith_rel_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_simple (input_phvs, output_phvs);
}
#[test]
fn test_simple_pred_raw_stateless_alu_arith_rel_cond_bool_2_2 () {
   let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_pred_raw_stateless_alu_arith_rel_cond_bool_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      simple_pred_raw_stateless_alu_arith_rel_cond_bool_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_simple (input_phvs, output_phvs);
}
#[test]
fn test_simple_sub_stateless_alu_arith_rel_2_2 () {
   let ticks : i32 = 100;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/simple_sub_stateless_alu_arith_rel_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      simple_sub_stateless_alu_arith_rel_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_simple (input_phvs, output_phvs);
}

/* Marple new flow tests */
#[test]
fn test_marple_new_flow_if_else_raw_stateless_alu_arith_2_2 () {
   let ticks : i32 = 1000;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/marple_new_flow_if_else_raw_stateless_alu_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      marple_new_flow_if_else_raw_stateless_alu_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_marple_new_flow (input_phvs, output_phvs);
}
#[test]
pub fn marple_new_flow_nested_ifs_stateless_alu_arith_rel_cond_2_2(){
   let ticks : i32 = 1000;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 1, 1);

  let hole_cfg_file : String = String::from("hole_configurations/marple_new_flow_nested_ifs_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      marple_new_flow_nested_ifs_stateless_alu_arith_rel_cond_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_marple_new_flow (input_phvs, output_phvs);
}
#[test]
fn test_marple_new_flow_pair_stateless_alu_arith_rel_cond_2_2 (){
  let ticks : i32 = 1000;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 2, 1);

  let hole_cfg_file : String = String::from("hole_configurations/marple_new_flow_pair_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      marple_new_flow_pair_stateless_alu_arith_rel_cond_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_marple_new_flow (input_phvs, output_phvs);
}
#[test]
fn test_marple_new_flow_pair_stateless_alu_arith_rel_cond_bool_3_3 () {
  let ticks : i32 = 1000;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 3, 1);

  let hole_cfg_file : String = String::from("hole_configurations/marple_new_flow_pair_stateless_alu_arith_rel_cond_bool_3_3_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      marple_new_flow_pair_stateless_alu_arith_rel_cond_bool_3_3_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_marple_new_flow (input_phvs, output_phvs);

}

/* blue increase tests */
#[test]
fn test_blue_increase_pair_stateless_alu_2_2() {
  let ticks : i32 = 1000;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 2, 2);

  let hole_cfg_file : String = String::from("hole_configurations/blue_increase_pair_stateless_alu_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      blue_increase_pair_stateless_alu_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_blue_increase (input_phvs, output_phvs);
}
#[test]
fn test_blue_increase_pair_stateless_alu_arith_2_2 (){
  let ticks : i32 = 1000;
  let mut input_phvs : Vec <Phv <i32> > = init_input_phvs(ticks, 2, 2, 2);

  let hole_cfg_file : String = String::from("hole_configurations/blue_increase_pair_stateless_alu_arith_2_2_hole_cfgs.txt");
  let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);
  let mut pipeline : Pipeline = 
      blue_increase_pair_stateless_alu_arith_2_2_prog_to_run::init_pipeline(hole_cfgs_map.clone());
  let output_phvs : Vec <Phv <i32> > = run_pipeline (input_phvs.clone(),
                                                     pipeline,
                                                     ticks);

  test_blue_increase (input_phvs, output_phvs);
}
/* Druzhba pipeline tests */
#[test]
fn test_basic_pipeline_1() {
    // state_vars is unused
    fn alu_stateless_fn( _state_vars: &mut Vec<StateVar>,
                         packet : &Vec<PhvContainer<i32>>) -> (Vec  <i32>, Vec <i32> ){
     
        (vec! [packet[0].field_value * 3], Vec::new())
    }

    // packet is unused
    fn alu_stateful_fn( state_vars: &mut Vec<StateVar>,
                        _packet : &Vec<PhvContainer<i32>>) -> (Vec <i32>, Vec <i32>) {
        let old_state : Vec <i32> = state_vars.clone();
        state_vars [0] = 10;
        (old_state, state_vars.clone())
    }

    // Picks the first phv container to input
    // into ALU 
    let alu_one_one_input_mux_index_hole = 0 ;
    let alu_one_two_input_mux_index_hole = 0 ;
    // Picks the third input which is the result from
    // the stateless ALU (state vars vector has 2 elements)
    let alu_one_one_output_mux_index_hole = 2 ;
    let alu_one_two_output_mux_index_hole = 2 ;

    let alu_two_one_input_mux_index_hole = 0 ;
    let alu_two_two_input_mux_index_hole = 0 ;
    let alu_two_one_output_mux_index_hole = 2 ;
    let alu_two_two_output_mux_index_hole = 2 ;

    //arbitrary state variables
    let state_vars : Vec <i32> = vec![0,1];

    /* Stage 1 */
    let first_phv : Phv<i32> = Phv{bubble: true, packets: Vec::new(), state : vec![vec![0;1]]};

    //generate input and output muxes for both ALUs in the first stage
    
    let alu_one_one_input_muxes : Vec<InputMux> = 
        vec![InputMux {input_phv: first_phv.clone() , index : alu_one_one_input_mux_index_hole}];
    let alu_one_two_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: first_phv.clone() , index : alu_one_two_input_mux_index_hole}];
    let alu_one_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_one_output_mux_index_hole};

    let alu_one_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_two_output_mux_index_hole};

    //generate ALUs for first pipeline stage
   
    let alu_one_one : ALU = 
        ALU { sequential_function: Box::new(alu_stateful_fn), 
              state_variables: state_vars.clone(), 
              input_muxes: alu_one_one_input_muxes, 
              output_mux: alu_one_one_output_mux, 
              is_stateful : true };
    let alu_one_two : ALU = 
        ALU { sequential_function: Box::new(alu_stateless_fn), 
              state_variables: Vec::new(), 
              input_muxes: alu_one_two_input_muxes, 
              output_mux: alu_one_two_output_mux, 
              is_stateful : false };
    
    let pipeline_stage_one : PipelineStage = PipelineStage { 
        stateful_atoms: vec![alu_one_one], 
        stateless_atoms : vec![alu_one_two],
        salu_configs : vec![0]};

    /* Stage 2 */
    let second_phv : Phv<i32> = Phv {bubble: true, packets: Vec::new(), state : vec![vec![0;1]]};

    //generate input and output muxes for both ALUs in the second stage
    
    let alu_two_one_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: second_phv.clone() , index : alu_two_one_input_mux_index_hole}];
    let alu_two_two_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: second_phv.clone() , index : alu_two_two_input_mux_index_hole}];
    let alu_two_one_output_mux : OutputMux = 
        OutputMux{input_phv_containers: Vec::new() , index: alu_two_one_output_mux_index_hole};
    let alu_two_two_output_mux : OutputMux = 
        OutputMux{input_phv_containers: Vec::new() , index: alu_two_two_output_mux_index_hole};

    //generate ALUs for second pipeline stage
    
    let alu_two_one : ALU = 
        ALU { sequential_function: Box::new(alu_stateful_fn), 
              state_variables: state_vars.clone(), 
              input_muxes : alu_two_one_input_muxes, 
              output_mux: alu_two_one_output_mux, 
              is_stateful : true };
    let alu_two_two : ALU = 
        ALU { sequential_function: Box::new(alu_stateless_fn), 
              state_variables: Vec::new(), 
              input_muxes : alu_two_two_input_muxes, 
              output_mux: alu_two_two_output_mux, 
              is_stateful: false 
        };

    let pipeline_stage_two : PipelineStage = PipelineStage{
        stateful_atoms: vec![alu_two_one], 
        stateless_atoms: vec![alu_two_two],
        salu_configs : vec![0],
    };
    //generate pipeline
    let mut pipeline : Pipeline = Pipeline::with_pipeline_stages(
        vec![pipeline_stage_one, pipeline_stage_two]);


    let field_values : Vec<i32> = vec![1, 2, 3, 4, 5, 6,
                                       7, 8, 9, 10, 11, 12, 
                                       13, 14, 15, 16, 17, 18,
                                       19, 20];
    
    let ticks : i32 = 20;
    let mut output_phvs : Vec<Phv<i32>> = Vec::new();
    for t in 0..ticks {
    
        let mut packet : Phv<i32> = Phv::new();

        packet.add_container_to_phv (PhvContainer {
            field_value : field_values [(t as usize)],
        });
        packet.set_state (vec![vec![0]]);
        let new_packet : Phv<i32> = pipeline.tick (packet);
        if !new_packet.is_bubble() {
          output_phvs.push (new_packet);
        }
        
    }
    // Assert that the PhvContainer in every Phv is equal
    // to the initial PhvContainer * 9. This is because we
    // have two pipeline stages each with a stateless ALU
    // that returns the Phv's first PhvContainer multiplied
    // by 3 and the output mux writes this value back to
    // that same PhvContainer.
    let mut index : usize = 0;
    for p in &output_phvs {
        assert! (p[0].get_value() == 
                 field_values [index]*9);
        index+=1;
    }
}
#[test]
fn test_basic_pipeline_2 () {
    // state_vars is unused
    fn alu_stateless_fn( _state_vars: &mut Vec<StateVar>,
                         packet : &Vec<PhvContainer<i32>>) -> ( Vec <i32>, Vec <i32>) {
        (vec! [packet[0].field_value * 2], Vec::new())
        
    }

    // packet is unused
    fn alu_stateful_fn( state_vars: &mut Vec<StateVar>,
                        _packet : &Vec<PhvContainer<i32>>) -> (Vec <i32>, Vec <i32>) {
        let old_state : Vec <i32> = state_vars.clone();
        state_vars [0] = state_vars[0] + 2;
        (old_state, state_vars.clone())
    }

    // Picks the first phv container to input
    // into ALU 
    let alu_one_one_input_mux_index_hole = 0 ;
    let alu_one_two_input_mux_index_hole = 0 ;
    // Picks the third input which is the result from
    // the stateless ALU (state vars vector has 2 elements)
    let alu_one_one_output_mux_index_hole = 2 ;
    let alu_one_two_output_mux_index_hole = 2 ;

    let alu_two_one_input_mux_index_hole = 0 ;
    let alu_two_two_input_mux_index_hole = 0 ;
    // Picks the first state variable
    let alu_two_one_output_mux_index_hole = 0 ;
    let alu_two_two_output_mux_index_hole = 0 ;

    //arbitrary state variables
    let mut state_vars : Vec<StateVar> = Vec::new();
    state_vars.push(0);
    state_vars.push(1);


    /* Stage 1 */
    let first_phv : Phv<i32> = Phv{bubble: true, packets: Vec::new(), state : vec![vec![0;1]]};

    //generate input and output muxes for both ALUs in the first stage
    
    let alu_one_one_input_muxes : Vec<InputMux> = 
        vec![InputMux {input_phv: first_phv.clone() , index : alu_one_one_input_mux_index_hole}];
    let alu_one_two_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: first_phv.clone() , index : alu_one_two_input_mux_index_hole}];
    let alu_one_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_one_output_mux_index_hole};

    let alu_one_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_two_output_mux_index_hole};

    //generate ALUs for first pipeline stage
   
    let alu_one_one : ALU = 
        ALU { sequential_function: Box::new(alu_stateful_fn), 
              state_variables: state_vars.clone(), 
              input_muxes: alu_one_one_input_muxes, 
              output_mux: alu_one_one_output_mux, 
              is_stateful : true };
    let alu_one_two : ALU = 
        ALU { sequential_function: Box::new(alu_stateless_fn), 
              state_variables: Vec::new(), 
              input_muxes: alu_one_two_input_muxes, 
              output_mux: alu_one_two_output_mux, 
              is_stateful : false };
    
    let pipeline_stage_one : PipelineStage = PipelineStage { 
        stateful_atoms: vec![alu_one_one], 
        stateless_atoms : vec![alu_one_two],
        salu_configs : vec![0],
    };

    /* Stage 2 */
    let second_phv : Phv<i32> = Phv {bubble: true, packets: Vec::new(), state : vec![vec![0;1]]};

    //generate input and output muxes for both ALUs in the second stage
    
    let alu_two_one_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: second_phv.clone() , index : alu_two_one_input_mux_index_hole}];
    let alu_two_two_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: second_phv.clone() , index : alu_two_two_input_mux_index_hole}];
    let alu_two_one_output_mux : OutputMux = 
        OutputMux{input_phv_containers: Vec::new() , index: alu_two_one_output_mux_index_hole};
    let alu_two_two_output_mux : OutputMux = 
        OutputMux{input_phv_containers: Vec::new() , index: alu_two_two_output_mux_index_hole};

    //generate ALUs for second pipeline stage
    
    let alu_two_one : ALU = 
        ALU { sequential_function: Box::new(alu_stateful_fn), 
              state_variables: state_vars.clone(), 
              input_muxes : alu_two_one_input_muxes, 
              output_mux: alu_two_one_output_mux, 
              is_stateful : true };
    let alu_two_two : ALU = 
        ALU { sequential_function: Box::new(alu_stateless_fn), 
              state_variables: Vec::new(), 
              input_muxes : alu_two_two_input_muxes, 
              output_mux: alu_two_two_output_mux, 
              is_stateful: false 
        };

    let pipeline_stage_two : PipelineStage = PipelineStage{
        stateful_atoms: vec![alu_two_one], 
        stateless_atoms: vec![alu_two_two],
        salu_configs : vec![0],
    };
    //generate pipeline
    let mut pipeline : Pipeline = Pipeline::with_pipeline_stages(
        vec![pipeline_stage_one, pipeline_stage_two]);


    let field_values : Vec<i32> = vec![1, 2, 3, 4, 5, 6,
                                       7, 8, 9, 10, 11, 12, 
                                       13, 14, 15, 16, 17, 18,
                                       19, 20];
    
    let ticks : i32 = 20;
    let mut output_phvs : Vec<Phv<i32>> = Vec::new();
    for t in 0..ticks {
    
        let mut packet : Phv<i32> = Phv::new();

        packet.add_container_to_phv (PhvContainer {
            field_value : field_values [(t as usize)],
        });
        packet.set_state(vec![vec![0]]);
        let new_packet : Phv<i32> = pipeline.tick (packet);
        if !new_packet.is_bubble() {
          output_phvs.push (new_packet);
        }
    }
    // Assert that the PhvContainer in every Phv is equal
    // to 2 * the index of the current phv. This is because 
    // the output muxes for the last pipeline stage have 
    // their hole values set to 0, which selects the state
    // first state variable. This state variable gets 
    // multiplied by 2 every time the stateful ALU gets 
    // run
    let mut count : i32 = 0;
    for p in &output_phvs {
        assert! (p[0].get_value() == 
                 count*2);
        count+=1;
    }
}

