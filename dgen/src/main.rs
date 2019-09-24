
pub mod rust_code_generator;
pub mod alu_parsing_utils;
// Important: nightly must be enabled to work
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub alugrammar); // synthesized by LALRPOP

use std::env; 
use alu_parsing_utils::AluParsingUtils;
use std::fs;

pub fn generate_alus (name : String,
                      stateful_file : String,
                      stateless_file : String,
                      pipeline_depth : i32,
                      pipeline_width : i32,
                      num_stateful_alus : i32,
                      constant_vec : Vec<i32>,
                      file_path : String)
{
  // Stateful AluParsingUtils initialization
  let stateful_alu = fs::read_to_string(&stateful_file)
    .expect("Something went wrong reading the file");
  let mut full_stateful_alu : AluParsingUtils = 
      AluParsingUtils::new(0, 0, 
                      name.clone(), 
                      stateful_alu,
                      true,
                      constant_vec.clone());

  // Stateless AluParsingUtils initialization
  let stateless_alu = fs::read_to_string(&stateless_file)
    .expect("Something went wrong reading the file");

  let mut full_stateless_alu : AluParsingUtils = 
      AluParsingUtils::new(0, 0, 
                      name.clone(), 
                      stateless_alu,
                      false,
                      constant_vec.clone());


  let mut pipeline_alus_string : String = String::from("");
  // Iterate through the pipeline depth and width and parse
  // all ALUs and their corresponding helper functions
  for _i in 0..pipeline_depth {
    for _j in 0..num_stateful_alus {
      let stateful_alu_string : String = 
        match alugrammar::AluParser::new().parse(
            &full_stateful_alu.prepend_opt_header_to_alu()){
          Ok (s) => s.to_string(),
          _      => panic!("Parsing stateful ALU failed"),
        };

      // ALU stateful helper functions
      let stateful_helper_string : String = rust_code_generator::get_helper_string();
      pipeline_alus_string.push_str (&stateful_helper_string);
      pipeline_alus_string.push_str (&stateful_alu_string);
      full_stateful_alu.increment_alu_count();
    }

    full_stateful_alu.reset_alu_count();
    full_stateful_alu.increment_pipeline_stage();
    for _j in 0..pipeline_width {

      let stateless_alu_string : String =  
        match alugrammar::AluParser::new().parse(
            &full_stateless_alu.prepend_opt_header_to_alu()){
          Ok (s) => s.to_string(),
          _      => panic! ("Parsing stateless ALU failed"),
        };
      // ALU stateless helper functions
      let stateless_helper_string : String = rust_code_generator::get_helper_string();
      pipeline_alus_string.push_str (&stateless_helper_string);
      pipeline_alus_string.push_str (&stateless_alu_string);
      full_stateless_alu.increment_alu_count();
    }
    full_stateless_alu.reset_alu_count();
    full_stateless_alu.increment_pipeline_stage();
  }
  // Contains all of the necessary use statements
  let file_imports : String = String::from ("use druzhba::phv_container::PhvContainer;\nuse druzhba::pipeline_stage::PipelineStage;\nuse druzhba::pipeline::Pipeline;\nuse druzhba::alu::ALU;\nuse druzhba::input_mux::InputMux;\nuse druzhba::output_mux::OutputMux;use druzhba::phv::Phv;\nuse std::collections::HashMap;\n");

  let init_pipeline : String = generate_init_pipeline (name.clone(), 
                                                       pipeline_depth, 
                                                       pipeline_width,
                                                       full_stateful_alu.get_number_of_operands(),
                                                       full_stateless_alu.get_number_of_operands(),
                                                       full_stateful_alu.get_number_of_state_variables(),
                                                       num_stateful_alus);
  let alu_data : String = generate_alu_data_functions (name.clone(), 
                                                       pipeline_depth, 
                                                       pipeline_width,
                                                       full_stateful_alu.get_number_of_operands(),
                                                       full_stateless_alu.get_number_of_operands(),
                                                       full_stateful_alu.get_number_of_state_variables(),
                                                       num_stateful_alus);

  let file_string : String = format! ("{}{}{}{}", 
                                      file_imports, 
                                      alu_data,
                                      pipeline_alus_string, 
                                      init_pipeline);

    fs::write(file_path, file_string)
      .expect("Error writing to prog_to_run.rs");


}

// Generates the init_pipeline() function in the prog_to_run.rs
// file. Druzhba will call init_pipeline() which will return a 
// packaged Pipeline with all of the pipeline stages and ALUs created.
// It will pass in a HashMap <String, i32> of hole names to values 
// which will then get plugged into the proper muxes and functions
fn generate_init_pipeline (name : String,
                           pipeline_depth : i32,
                           pipeline_width : i32,
                           num_stateful_operands : i32,
                           num_stateless_operands : i32,
                           num_state_variables : i32,
                           num_stateful_alus : i32) -> String {

  let mut pipeline : String = String::from
      ("pub fn init_pipeline (hole_vars : HashMap <String, i32>) -> Pipeline { \n");
  pipeline.push_str 
      ("  let mut pipeline_stages : Vec<PipelineStage> = Vec::new();\n");
  for i in 0..pipeline_depth {
 
    let mut pipeline_stage : String = format!("\n  // Stage {} stateful ALUs\n",i);

    // Vectors to hold alus before initializing PipelineStage
    let stateful_alus_vec : String = format!("stateful_alus_{}", i);
    let stateless_alus_vec : String = format!("stateless_alus_{}",i);
    pipeline_stage.push_str (&format!
                             ("  let mut {} : Vec <ALU> = Vec::new();\n",
                             stateful_alus_vec));

    pipeline_stage.push_str (&format!
                             ("  let mut {} : Vec <ALU> = Vec::new();\n", 
                             stateless_alus_vec));

    for j in 0..num_stateful_alus {
      // init_alu function name
      let stateful_init_alu_name : String = format!("init_{}_stateful_alu_{}_{}", 
                                                  &name, i, j);

      // Variable names
      let stateful_alu_name : String = format!("stateful_alu_{}_{}", i, j);
      let state_variables : String = format!("state_variables_{}_{}", i, j);
      let stateful_input_muxes : String = 
          format! ("stateful_input_muxes_{}_{}", i, j);
      let stateful_output_mux : String = 
          format! ("stateful_output_mux_{}_{}", i, j);
      pipeline_stage.push_str (&format!("  let mut {} : Vec<InputMux> = Vec::new();\n", 
                                      stateful_input_muxes));

      pipeline_stage.push_str (&format!("  let empty_phv : Phv <i32> = Phv {{ bubble : true, packets : Vec::new(), state : Vec::new() }};\n"));

      // Iterates from 0 to number of stateful operands and creates
      // an input mux for each of them
      for k in 0..num_stateful_operands {
        // Input mux ALU count value hardcoded to 0 because only
        // 1 stateful ALU per stage
        let input_mux_hole : String = format!("{}_stateful_operand_mux_{}_{}_{}_ctrl", 
                                              &name, i, j, k);
        pipeline_stage.push_str (&format!("  {}.push (InputMux {{ input_phv : empty_phv.clone(), index : hole_vars[\"{}\"] }});\n", 
                                          stateful_input_muxes,
                                          input_mux_hole));
      }

      pipeline_stage.push_str ("  // No hole variables for stateful ALU OutputMux\n");
  //    Blank output mux. This mux is not actually used, it's just 
  //    to initilize ALU
      pipeline_stage.push_str (&format!("  let {} : OutputMux = OutputMux {{input_phv_containers : Vec::new(), index : 0 }};\n",
      stateful_output_mux));

      // Initializes a vector of all 0's for state vector
      pipeline_stage.push_str 
          (&format!("  let {} : Vec<i32> = vec![0; {}];\n", 
           state_variables, num_state_variables));

      pipeline_stage.push_str
          (&format! ("  let {} : ALU = ALU {{sequential_function : {}(hole_vars.clone()), state_variables : {}, input_muxes : {}, output_mux : {}, is_stateful: true }};\n", 
          stateful_alu_name, stateful_init_alu_name, state_variables, stateful_input_muxes, stateful_output_mux));

      pipeline_stage.push_str (&format!("  {}.push({});", stateful_alus_vec, stateful_alu_name));
    }
    pipeline_stage.push_str(&format!("\n\n  // Stage {} stateless ALUs\n", i));
    // Generate stateless ALU initializations
    for j in 0..pipeline_width {

      let stateless_init_alu_name : String = 
          format!("init_{}_stateless_alu_{}_{}", &name, i, j);
      let stateless_alu_name : String = 
          format!("stateless_alu_{}_{}", i, j);
      let stateless_input_muxes : String = 
          format! ("stateless_input_muxes_{}_{}", i, j);
      let stateless_output_mux : String = 
          format! ("stateless_output_mux_{}_{}", i, j);

      pipeline_stage.push_str (&format!("  let mut {} : Vec<InputMux> = Vec::new();\n", 
                                        stateless_input_muxes));
      // Goes from 1 to the number of operands and initializes
      // InputMuxes. Start from 1 because it does so in 
      // Chipmunk for the mux counter
      for k in 1..num_stateless_operands+1 {

        let input_mux_hole : String = format!("{}_stateless_alu_{}_{}_mux{}_ctrl", 
                                     &name, i, j, k);
      pipeline_stage.push_str (&format!("  {}.push (InputMux {{ input_phv : empty_phv.clone(), index : hole_vars[\"{}\"] }});\n", 
                                        stateless_input_muxes,
                                        input_mux_hole));

      }
      // Actual output mux that will be use generated here
      let stateless_output_mux_hole : String = format!("{}_output_mux_phv_{}_{}_ctrl",
                                             &name, i, j);
      pipeline_stage.push_str (&format!("  let {} : OutputMux = OutputMux {{ input_phv_containers : Vec::new(), index : hole_vars[\"{}\"]}};\n", 
                                        stateless_output_mux,
                                        stateless_output_mux_hole));


    pipeline_stage.push_str  (
        &format! ("  let {} : ALU = ALU {{sequential_function : {}(hole_vars.clone()), state_variables : Vec::new(), input_muxes : {}, output_mux : {}, is_stateful: false }};\n",
        stateless_alu_name, stateless_init_alu_name, stateless_input_muxes, stateless_output_mux));

    // Add the stateless ALU to vector
    pipeline_stage.push_str 
        (&format!("  {}.push({});\n", 
         stateless_alus_vec, stateless_alu_name));
                

    }
    let salu_configs_string : String = format!("salu_configs_{}",
                                              i);
    let mut salu_configs : String = format!("  let {} : Vec <i32> = vec![",
                                            salu_configs_string);
    for k in 0..num_stateful_alus {
      salu_configs.push_str(&format!("hole_vars[\"{}_salu_config_{}_{}\"]",
                                     name.clone(),
                                     i,
                                     k));
      if k < num_stateful_alus - 1 {
        salu_configs.push_str(",");
      }
 
    }
   salu_configs.push_str("];\n");
    
   let output_mux_globals_string : String = format!("output_mux_globals_{}",
                                                    i);
   let mut output_mux_globals : String = format!("  let {} : Vec <i32> = vec![",
                                             output_mux_globals_string);
   for k in 0..num_stateful_alus {
     output_mux_globals.push_str (&format!("hole_vars[\"{}_stateful_alu_{}_{}_output_mux_global\"]",
                                           name.clone(),
                                           i,
                                           k));
     if k < num_stateful_alus - 1 {
      output_mux_globals.push_str(",");
     }
   }

   output_mux_globals.push_str("];\n");
   pipeline_stage.push_str (&salu_configs);

   pipeline_stage.push_str (&output_mux_globals);
    // Initialize pipeline stage
    pipeline_stage.push_str 
        (&format!("  let pipeline_stage_{} : PipelineStage = PipelineStage {{stateful_atoms : {}, stateless_atoms : {} , salu_configs : {}, output_mux_globals : {} }};\n",
                  i, 
                  stateful_alus_vec, 
                  stateless_alus_vec,
                  salu_configs_string,
                  output_mux_globals_string));
    pipeline_stage.push_str 
        (&format!("  pipeline_stages.push(pipeline_stage_{});\n",i));
    pipeline.push_str (&pipeline_stage);

  }
  pipeline.push_str ("\n  // Initializing Pipeline using all PipelineStages \n");
  pipeline.push_str ("  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);\n");
  pipeline.push_str ("  pipeline\n}\n");
  
  pipeline
}
fn generate_alu_data_functions (name : String,
                                pipeline_depth : i32,
                                pipeline_width : i32,
                                num_stateful_operands : i32,
                                num_stateless_operands : i32,
                                num_state_variables : i32,
                                num_stateful_alus : i32 ) -> String{
  let name_fn : String = 
      format!("pub fn name() -> String {{\n  \"{}\".to_string()\n}}\n",
      name);
  let pipeline_depth_fn : String = 
      format!("pub fn pipeline_depth () -> i32 {{\n  {}\n}}\n",
      pipeline_depth);
  let pipeline_width_fn : String = 
      format!("pub fn pipeline_width () -> i32 {{\n  {}\n}}\n",
      pipeline_width);

  let num_stateful_operands_fn : String = 
      format!("pub fn num_stateful_operands () -> i32 {{\n  {}\n}}\n",
      num_stateful_operands);
  let num_stateless_operands_fn : String = 
      format!("pub fn num_stateless_operands () -> i32 {{\n  {}\n}}\n",
      num_stateless_operands);
  let num_state_variables_fn : String =
      format!("pub fn num_state_variables() -> i32 {{\n  {}\n}}\n",
      num_state_variables);
  let num_stateful_alus_fn : String = 
      format!("pub fn num_stateful_alus() -> i32 {{\n  {}\n}}\n",
      num_stateful_alus);

  // Assert number of virtual stateful ALUs per pipeline stage
  // is less than or equal to pipeline width. In other words,
  // virtual stateful ALUs cannot exceed number of physical
  // stateful ALUs
  assert!(num_stateful_alus <= pipeline_width);
  format!("{}{}{}{}{}{}{}",
          name_fn,
          pipeline_depth_fn,
          pipeline_width_fn,
          num_stateful_operands_fn,
          num_stateless_operands_fn,
          num_state_variables_fn,
          num_stateful_alus_fn)

  
}
fn main() {
    let args : Vec<String> = env::args().collect();
    // Make room for optional output file path
    assert! (args.len() == 8 || args.len() == 9);

    let spec_name : String = args[1].clone();
    let stateful_alu : String = 
        format!("{}", args[2].clone());
    let stateless_alu : String = 
        format!("{}", args[3].clone());
    let pipeline_depth : i32 = 
        match args[4].parse::<i32>() {
          Ok (t_pipeline_depth) => t_pipeline_depth,
          Err (_)               => panic!("Failure: Unbale to unwrap pipeline depth"),
        };
    let pipeline_width : i32 = 
        match args[5].parse::<i32>() {
          Ok (t_pipeline_width) => t_pipeline_width,
          Err (_)               => panic!("Failure: Unbale to unwrap pipeline depth"),
        };
    let num_stateful_alus : i32 =  
        match args[6].parse::<i32>() {
          Ok (t_pipeline_width) => t_pipeline_width,
          Err (_)               => panic!("Failure: Unbale to unwrap number of stateful ALUs"),
        };
    let constant_vec_string : String = args[7].clone();
    let constant_vec : Vec <i32> = constant_vec_string
                                   .split(",")
                                   .map(|n| 
                                        match n.trim().parse::<i32>() {
                                          Ok (num) => num,
                                          Err (_)  => panic!("Failrure: Unable to parse constant set"),
                                   })
                                    .collect();
    let mut file_path = String::from("src/prog_to_run.rs");
    if args.len() == 9 {
        file_path = format!("{}", args[8].clone());
    }
    
    println!("Constant set: {:?}", constant_vec);

    let stateful_alu_split : Vec <String>= stateful_alu.split("/")
                                                       .map (|s| s.to_string())
                                                       .collect();
    let stateless_alu_split : Vec <String>= stateless_alu.split("/")
                                                       .map (|s| s.to_string())
                                                       .collect();


    let full_stateful_name = &stateful_alu_split[stateful_alu_split.len()-1].to_string();

    let stateful_name = &full_stateful_name[0..full_stateful_name.len()-4].to_string();
    let full_stateless_name = &stateless_alu_split[stateless_alu_split.len()-1].to_string();
    let stateless_name = &full_stateless_name[0..full_stateless_name.len()-4].to_string();
    let name : String = format!("{}_{}_{}_{}_{}", 
                                spec_name,
                                stateful_name,
                                stateless_name,
                                pipeline_depth,
                                pipeline_width);

    generate_alus (name, 
                   stateful_alu, 
                   stateless_alu, 
                   pipeline_depth, 
                   pipeline_width,
                   num_stateful_alus,
                   constant_vec,
                   file_path);
}

#[cfg(test)]
mod test_grammar;

