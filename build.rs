use std::fs;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() { 
    
  let out_dir = String::from("src/");;
  let destination = Path::new(&out_dir).join("test_with_chipmunk.rs");
  let mut test_file = File::create(&destination).unwrap();

  let test_case_names : Vec <String> = 
      vec!["blue_increase_pair_stateless_alu_arith_4_2".to_string()];
   Command::new("mkdir")
           .arg("src/tests")
           .output()
           .expect("Could not create tests directory");

  write_mod_file (&test_case_names);
  run_dgen (&test_case_names);
   // write test file header, put `use`, `const` etc there
  write_header(&mut test_file, &test_case_names);
  let test_data_directory = read_dir("src/tests/").unwrap();
  for dgen_output_file in test_data_directory {
    write_test(&mut test_file, &dgen_output_file.unwrap());
  }
}

// Runs dgen multiple times to produce all of the prog_to_run.rs
// files needed for the tests
fn run_dgen (test_case_names : &Vec<String>)
{

  let dgen_args : Vec <Vec <String> > = vec![
    vec! ["blue_increase".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3,6,4,5,9".to_string()],

  ];
  Command::new("cp")
           .arg("dgen/target/debug/dgen")
           .arg("dgen_bin")
           .output()
           .expect("Copying dgen binary failed. Ensure that dgen is compiled first");
  Command::new("chmod")
           .arg("+x")
           .arg("dgen_bin")
           .output()
           .expect("Adding execution permissions to dgen_bin failed");
  let mut index : usize = 0;
  for arg in dgen_args.iter(){
     
    Command::new("./dgen_bin")
             .arg(&arg[0])
             .arg(&arg[1])
             .arg(&arg[2])
             .arg(&arg[3])
             .arg(&arg[4])
             .arg(&arg[5])
             .arg(&arg[6])
             .arg(format!("src/tests/{}.rs", test_case_names[index]))
             .output()
             .expect("Error running dgen");
    index+=1;
  }
  // Cleanup
  Command::new("rm")
           .arg("dgen_bin")
           .output()
           .expect("Removing dgen binary failed");
}

fn write_mod_file (test_case_names : &Vec<String>) {

  let mut declaration_list : String = String::from("");
  for n in test_case_names.iter(){
    declaration_list.push_str (&format!("pub mod {};\n",
                                         n));
  }
  fs::write("src/tests/mod.rs", declaration_list)
     .expect("Error writing to mod.rs");
}
// Fills out the contents of the test file
fn write_test(test_file: &mut File, dgen_output_file : &DirEntry) {
    let dgen_output_file = dgen_output_file.path().canonicalize().unwrap();
    let path = dgen_output_file.display();
    println!("File: {}", dgen_output_file.file_name().unwrap().to_string_lossy());
    let test_name = format!(
                    "prefix_if_needed_{}",
                    dgen_output_file.file_name().unwrap().to_string_lossy()
                                        );

    write!(test_file, include_str!("./test/test_template"),
                      name = test_name,
                      path = path.to_string()
                      ).expect("Error writing to test_with_chipmunk.rs");
}

// Writes all of the imports to the top of the test file
fn write_header(test_file: &mut File, 
                test_case_names : &Vec<String>) {

  let mut test_case_imports : String = String::from("");
  for n in test_case_names.iter(){
    test_case_imports.push_str (&format!("use tests::{};\n",
                                         n));
  }
  let full_import_list : String = format!("extern crate druhzba;\nuse druzhba::output_mux::OutputMux;\nuse druzhba::input_mux::InputMux;\nuse druzhba::pipeline::Pipeline;\nuse druzhba::phv::Phv;\nuse druzhba::phv_container::PhvContainer;\nuse druzhba::pipeline_stage::PipelineStage;\nuse druzhba::alu::ALU;\nuse druzhba::alu::StateVar;\nuse rand::Rng;\nuse std::fs;\nuse std::collections::HashMap;\n{}", test_case_imports);


  write!(test_file, "{}", full_import_list).expect("Error writing to test_with_chimunk.rs header");
}
