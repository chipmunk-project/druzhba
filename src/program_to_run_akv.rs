use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;
use std::collections::HashMap;

pub fn alu_fn(  state_vars: &mut HashMap<String, i32>,
                packet : &mut Vec<PhvContainer<i32>>  ){

        packet[0].field_value = packet[0].field_value * 2; 
        //state_vars.insert("count".to_string(), *state_vars.get_mut("count").unwrap() += 1);
}


pub fn init_pipeline() -> Pipeline {

    /*Holes provided by *Chipmunk* for first stage input and output muxes.
     Eventually, values should be read in from a file*/

    let alu_one_one_input_mux_index_hole = 0 ;
    let alu_one_two_input_mux_index_hole = 1 ;
    let alu_one_one_output_mux_index_hole = 0 ;
    let alu_one_two_output_mux_index_hole = 0 ;

    let alu_two_one_input_mux_index_hole = 0 ;
    let alu_two_two_input_mux_index_hole = 1 ;
    let alu_two_one_output_mux_index_hole = 0 ;
    let alu_two_two_output_mux_index_hole = 0 ;

    //arbitrary state vars
    let mut state_vars : HashMap<String,i32> = HashMap::new();
    state_vars.insert("count".to_string(), 1);
    state_vars.insert("arbitrary".to_string(), 1);

    //Load an initial PHV with two containers, src_ip and dst_ip
//     let src_ip : PhvContainer<i32> = PhvContainer{field_value: 1};
//     let dst_ip : PhvContainer<i32> = PhvContainer{field_value: 2};
    let first_phv : Phv<i32> = Phv{bubble: true, packets: Vec::new()};

    //generate input and output muxes for both ALUs in the first stage
    let alu_one_one_input_mux : InputMux = InputMux{input_phv: first_phv.clone() , index : alu_one_one_input_mux_index_hole};
    let alu_one_two_input_mux : InputMux= InputMux{input_phv: first_phv.clone() , index : alu_one_two_input_mux_index_hole};

    let alu_one_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_one_output_mux_index_hole};
    let alu_one_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_two_output_mux_index_hole};

    //generate ALUs for first (and only for now) pipeline stage
    let alu_one_one : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_one_one_input_mux, output_mux: alu_one_one_output_mux};
    let alu_one_two : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_one_two_input_mux, output_mux: alu_one_two_output_mux};
    
    let pipeline_stage_one : PipelineStage = PipelineStage{ atoms: vec![alu_one_one, alu_one_two] };

    let second_phv : Phv<i32> = Phv {bubble: true, packets: Vec::new()};

    let alu_two_one_input_mux : InputMux = InputMux{input_phv: second_phv.clone() , index : alu_two_one_input_mux_index_hole};
    let alu_two_two_input_mux : InputMux = InputMux{input_phv: second_phv.clone() , index : alu_two_two_input_mux_index_hole};
    
    let alu_two_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_two_one_output_mux_index_hole};
    let alu_two_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_two_two_output_mux_index_hole};

    let alu_two_one : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_two_one_input_mux, output_mux: alu_two_one_output_mux};
    let alu_two_two : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_two_two_input_mux, output_mux: alu_two_two_output_mux};

    let pipeline_stage_two : PipelineStage = PipelineStage{atoms: vec![alu_two_one, alu_two_two]};

    let pipeline : Pipeline = Pipeline::with_pipeline_stages(vec![pipeline_stage_one, pipeline_stage_two]);

    pipeline

}