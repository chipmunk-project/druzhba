
pub mod ast;
pub mod alu_to_parse;
// Important: nightly must be enabled to work
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub alugrammar); // synthesized by LALRPOP

use std::env; 
use alu_to_parse::AluToParse;
use std::fs;

pub fn generate_alus (name : String,
                      stateful_file : String,
                      stateless_file : String,
                      pipeline_depth : i32,
                      pipeline_width : i32)
{

  // Stateful AluToParse initialization
  let stateful_alu = fs::read_to_string(&stateful_file)
    .expect("Something went wrong reading the file");
  //println!("{}", stateful_alu);
  let mut full_stateful_alu : AluToParse = 
      AluToParse::new(0, 0, 
                      name.clone(), 
                      stateful_alu,
                      true);

  // Stateless AluToParse initialization
  let stateless_alu = fs::read_to_string(&stateless_file)
    .expect("Something went wrong reading the file");

  let mut full_stateless_alu : AluToParse = 
      AluToParse::new(0, 0, 
                      name.clone(), 
                      stateless_alu,
                      false);


  let mut pipeline_alus_string : String = String::from("");
  // Iterate through the pipeline depth and width and parse
  // all ALUs and their corresponding helper functions
  for _i in 0..pipeline_depth {
    let stateful_alu_string : String = 
      match alugrammar::AluParser::new().parse(
          &full_stateful_alu.get_string_to_parse()){
        Ok (s) => s.to_string(),
        _      => panic!("Parsing stateful ALU failed"),
      };

    // ALU stateful helper functions
    let stateful_helper_string : String = ast::get_helper_string();
    pipeline_alus_string.push_str (&stateful_helper_string);
    pipeline_alus_string.push_str (&stateful_alu_string);
    full_stateful_alu.increment_pipeline_stage();

    for _j in 0..pipeline_width {

      let stateless_alu_string : String =  
        match alugrammar::AluParser::new().parse(
            &full_stateless_alu.get_string_to_parse()){
          Ok (s) => s.to_string(),
          _      => panic! ("Parsing stateless ALU failed"),
        };
      // ALU stateless helper functions
      let stateless_helper_string : String = ast::get_helper_string();
      pipeline_alus_string.push_str (&stateless_helper_string);
      pipeline_alus_string.push_str (&stateless_alu_string);
      full_stateless_alu.increment_alu_count();
    }
    full_stateless_alu.reset_alu_count();
    full_stateless_alu.increment_pipeline_stage();
  }
  // Contains all of the necessary use statements
  let file_intro : String = String::from ("use druzhba::phv_container::PhvContainer;\nuse druzhba::pipeline_stage::PipelineStage;\nuse druzhba::pipeline::Pipeline;\nuse druzhba::alu::ALU;\nuse druzhba::input_mux::InputMux;\nuse druzhba::output_mux::OutputMux;use druzhba::phv::Phv;\nuse std::collections::HashMap;\n");

    let init_pipeline : String = String::from("\npub fn init_pipeline () -> Pipeline {Pipeline::with_pipeline_stages(Vec::new())}\n");
    let file_string : String = format! ("{}{}{}", 
                                      file_intro, 
                                      pipeline_alus_string,
                                      init_pipeline);


  fs::write("src/prog_to_run.rs", file_string)
      .expect("Error writing to prog_to_run.rs");



}
fn main() {
    let args : Vec<String> = env::args().collect();
    assert! (args.len() == 6);

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

    generate_alus (name, stateful_alu, 
                   stateless_alu, pipeline_depth, pipeline_width);
}

#[cfg(test)]
mod test_grammar;

