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
}


pub fn init_pipeline() -> Pipeline {

    /*Holes provided by *Chipmunk* for input and output muxes of two stages. */
    
    let alu_one_one_input_mux_index_hole = 0 ;
    let alu_one_two_input_mux_index_hole = 1 ;
    let alu_one_one_output_mux_index_hole = 0 ;
    let alu_one_two_output_mux_index_hole = 0 ;

    let alu_two_one_input_mux_index_hole = 0 ;
    let alu_two_two_input_mux_index_hole = 1 ;
    let alu_two_one_output_mux_index_hole = 0 ;
    let alu_two_two_output_mux_index_hole = 0 ;

    //arbitrary state variables
    let mut state_vars : HashMap<String,i32> = HashMap::new();
    state_vars.insert("count".to_string(), 1);
    state_vars.insert("arbitrary".to_string(), 1);


    /* Stage 1 */
    let first_phv : Phv<i32> = Phv{bubble: true, packets: Vec::new()};

    //generate input and output muxes for both ALUs in the first stage
    
    let alu_one_one_input_mux : InputMux = InputMux{input_phv: first_phv.clone() , index : alu_one_one_input_mux_index_hole};
    let alu_one_two_input_mux : InputMux= InputMux{input_phv: first_phv.clone() , index : alu_one_two_input_mux_index_hole};
    let alu_one_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_one_output_mux_index_hole};
    let alu_one_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_one_two_output_mux_index_hole};

    //generate ALUs for first pipeline stage
   
    let alu_one_one : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_one_one_input_mux, output_mux: alu_one_one_output_mux};
    let alu_one_two : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_one_two_input_mux, output_mux: alu_one_two_output_mux};
    
    let pipeline_stage_one : PipelineStage = PipelineStage{ atoms: vec![alu_one_one, alu_one_two] };

    /* Stage 2 */
    let second_phv : Phv<i32> = Phv {bubble: true, packets: Vec::new()};

    //generate input and output muxes for both ALUs in the second stage
    
    let alu_two_one_input_mux : InputMux = InputMux{input_phv: second_phv.clone() , index : alu_two_one_input_mux_index_hole};
    let alu_two_two_input_mux : InputMux = InputMux{input_phv: second_phv.clone() , index : alu_two_two_input_mux_index_hole};
    let alu_two_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_two_one_output_mux_index_hole};
    let alu_two_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_two_two_output_mux_index_hole};

    //generate ALUs for second pipeline stage
    
    let alu_two_one : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_two_one_input_mux, output_mux: alu_two_one_output_mux};
    let alu_two_two : ALU = ALU{sequential_function: alu_fn, state_variables: state_vars.clone(), input_mux: alu_two_two_input_mux, output_mux: alu_two_two_output_mux};

    let pipeline_stage_two : PipelineStage = PipelineStage{atoms: vec![alu_two_one, alu_two_two]};

    //generate pipeline
    
    let pipeline : Pipeline = Pipeline::with_pipeline_stages(vec![pipeline_stage_one, pipeline_stage_two]);

    pipeline

}