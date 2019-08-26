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

use rand::Rng;
use std::fs;
use std::collections::HashMap;
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

