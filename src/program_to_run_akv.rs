use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::alu::ALU;
use druzhba::alu::StateVar;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;


//Stateless ALU

pub fn alu_stateless_fn( _state_vars: &mut Vec<StateVar>,
                         packet : &Vec<PhvContainer<i32>>) -> Vec <i32>{
 
    vec! [packet[0].field_value * 21]
}

pub fn alu_stateful_fn( state_vars: &mut Vec<StateVar>,
                       _packet : &Vec<PhvContainer<i32>>) -> Vec <i32>{
    let old_state : Vec <i32> = state_vars.clone();
    state_vars [0] = 21;
    old_state
}



pub fn init_pipeline() -> Pipeline {

    /*Holes provided by *Chipmunk* for input and output muxes of two stages. */
    
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
    let mut state_vars : Vec<StateVar> = Vec::new();
    state_vars.push(0);
    state_vars.push(1);


    /* Stage 1 */
    let first_phv : Phv<i32> = Phv{bubble: true, packets: Vec::new()};

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
    
    let pipeline_stage_one : PipelineStage = PipelineStage{ 
        stateful_atoms: vec![alu_one_one], 
        stateless_atoms : vec![alu_one_two] };

    /* Stage 2 */
    let second_phv : Phv<i32> = Phv {bubble: true, packets: Vec::new()};

    //generate input and output muxes for both ALUs in the second stage
    
    let alu_two_one_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: second_phv.clone() , index : alu_two_one_input_mux_index_hole}];
    let alu_two_two_input_muxes : Vec<InputMux> = 
        vec![InputMux{input_phv: second_phv.clone() , index : alu_two_two_input_mux_index_hole}];
    let alu_two_one_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_two_one_output_mux_index_hole};
    let alu_two_two_output_mux : OutputMux = OutputMux{input_phv_containers: Vec::new() , index: alu_two_two_output_mux_index_hole};

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
        stateless_atoms: vec![alu_two_two]
    };

    //generate pipeline

    let pipeline : Pipeline = Pipeline::with_pipeline_stages(vec![pipeline_stage_one, pipeline_stage_two]);

    pipeline

}
