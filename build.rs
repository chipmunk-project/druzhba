use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

// To add a new test to the test suite, insert the name
// into test_case_names and fill out the necessary data
// in dgen_data
fn main() { 
    
  let out_dir = String::from("src/");;
  let destination = Path::new(&out_dir).join("test_with_chipmunk.rs");
  let mut test_file = File::create(&destination).unwrap();

  let test_case_names : Vec <String> = test_names();
  let dgen_data : Vec <Vec <String> > = test_configurations();
   Command::new("mkdir")
           .arg("src/tests")
           .output()
           .expect("Could not create tests directory");

  write_mod_file (&test_case_names);
  run_dgen (&test_case_names, &dgen_data);
   // write test file header, put `use`, `const` etc there
  write_header(&mut test_file, &test_case_names);
  let test_data_directory = read_dir("src/tests/").unwrap();
  let mut index : usize = 0;

  for dgen_output_file in test_data_directory {

      let file_name = match dgen_output_file {
        Ok (f) => format!("{:?}", f.file_name()),
        Err (_)      => panic!("Unable to unwrap test file"),
      };

      if file_name.contains ("mod.rs") || index >= dgen_data.len() ||
          index >= test_case_names.len(){
        continue;
      }
    write_test(&mut test_file, 
               &dgen_data[index],
               test_case_names[index].clone());
    index+=1;
  }
}

// Runs dgen multiple times to produce all of the prog_to_run.rs
// files needed for the tests
fn run_dgen (test_case_names : &Vec<String>,
             dgen_args : &Vec <Vec<String>>)
{
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
    
    let status = Command::new("./dgen_bin")
                 .arg(&arg[0]) // Name
                 .arg(&arg[1]) // Stateful ALU
                 .arg(&arg[2]) // Stateless ALU
                 .arg(&arg[3]) // Depth
                 .arg(&arg[4]) // Width
                 .arg(&arg[8]) // Stateful ALUs
                 .arg(&arg[5]) // constant vec
                 .arg(format!("src/tests/{}.rs", test_case_names[index]))
                 .output()
                 .expect("Error running dgen");
    println!("{} status: {:?}\n", test_case_names[index], status);
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
fn write_test(test_file: &mut File, 
              dgen_data : &Vec <String>,
              test_name : String) {

    write!(test_file, include_str!("./test/test_template"),
                      name = format!("test_{}",test_name),
                      num_packets = dgen_data[6],
                      num_containers = dgen_data[4], 
                      num_state_vars = dgen_data[7],
                      num_stateful_alus = dgen_data[8],
                      hole_configurations = dgen_data[9],
                      prog_to_run_file = test_name,
                      test_function = format!("test_{}",dgen_data[10])
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
  let full_import_list : String = format!("extern crate druzhba;\nuse druzhba::output_mux::OutputMux;\nuse druzhba::input_mux::InputMux;\nuse druzhba::pipeline::Pipeline;\nuse druzhba::phv::Phv;\nuse druzhba::phv_container::PhvContainer;\nuse druzhba::pipeline_stage::PipelineStage;\nuse druzhba::alu::ALU;\nuse druzhba::alu::StateVar;\nuse rand::Rng;\nuse std::fs;\nuse std::collections::HashMap;\n{}", test_case_imports);


  write!(test_file, "{}", full_import_list).expect("Error writing to test_with_chimunk.rs header");

  write!(test_file, include_str!("./test/test_helper_functions")).
      expect("Error writing helper functions to test_with_chipmunk.rs");
}

fn test_names () -> Vec <String> {
  vec![
  "simple_raw_stateless_alu_2_2".to_string(),
  "simple_raw_stateless_alu_arith_3_4".to_string(),
  "simple_raw_stateless_alu_arith_2_3".to_string(),
  "simple_if_else_raw_stateless_alu_arith_rel_cond_2_2".to_string(),
  "simple_if_else_raw_stateless_alu_arith_rel_cond_bool_2_4".to_string(),
  "simple_sub_stateless_alu_arith_rel_cond_2_3".to_string(),
  "simple_nested_ifs_stateless_alu_arith_rel_2_2".to_string(),

  "simple_pair_stateless_alu_arith_2_4".to_string(),
  "marple_new_flow_pred_raw_stateless_alu_arith_rel_cond_3_2".to_string(),
  "marple_new_flow_sub_stateless_alu_arith_rel_cond_bool_2_3".to_string(),
  "marple_new_flow_pair_stateless_alu_2_4".to_string(),
  "marple_tcp_nmo_if_else_raw_stateless_alu_4_2".to_string(),
  "marple_tcp_nmo_nested_ifs_stateless_alu_arith_rel_3_2".to_string(),
  "blue_increase_pair_stateless_alu_arith_4_2".to_string(),

  "learn_filter_pred_raw_stateless_alu_arith_rel_2_3".to_string(),
  "learn_filter_if_else_raw_stateless_alu_2_3".to_string(),
  "learn_filter_sub_stateless_alu_arith_rel_cond_2_3".to_string(),
  "sampling_sub_stateless_alu_3_3".to_string(),
  "sampling_pair_stateless_alu_arith_rel_3_3".to_string(),
  "rcp_if_else_raw_stateless_alu_arith_rel_3_3".to_string(),
  "rcp_nested_ifs_stateless_alu_2_3".to_string(),
  "times_two_sub_stateless_alu_arith_3_3".to_string(),
  "times_two_if_else_raw_stateless_alu_arith_rel_4_2".to_string(),
  "test_pred_raw_stateless_alu_arith_rel_cond_bool_3_3".to_string(),
  "test_if_else_raw_stateless_alu_arith_rel_cond_4_4".to_string(),
  "snap_heavy_hitter_pair_stateless_alu_2_3".to_string(),
  "snap_heavy_hitter_pair_stateless_alu_arith_rel_2_2".to_string(),
  "sampling_revised_nested_ifs_stateless_alu_arith_rel_cond_2_2".to_string(),
  "sampling_revised_pair_stateless_alu_arith_rel_3_3".to_string(),
  // Experiment tests
  "blue_increase_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2".to_string(),
  "blue_increase_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2".to_string(),

   "blue_increase_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2".to_string(),
  "blue_increase_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2".to_string(),
 "blue_increase_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2".to_string(),
  "blue_increase_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_1_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_2_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_3_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
   "blue_decrease_equivalent_4_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_6_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_8_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),
  "blue_decrease_equivalent_9_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2".to_string(),

  "conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_2_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_3_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_4_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_5_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_6_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_7_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_8_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_9_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  "conga_equivalent_10_canonicalizer_equivalent_1_pair_stateless_alu_1_5".to_string(),
  
  "marple_new_flow_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),

  "marple_new_flow_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_new_flow_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_new_flow_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_new_flow_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  
  "marple_new_flow_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_new_flow_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_new_flow_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_new_flow_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),

  "marple_new_flow_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2".to_string(),
  "marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),

  "marple_tcp_nmo_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  "marple_tcp_nmo_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),

  "marple_tcp_nmo_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  "marple_tcp_nmo_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  "marple_tcp_nmo_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  "marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  
  "marple_tcp_nmo_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  "marple_tcp_nmo_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
  "marple_tcp_nmo_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2".to_string(),
"learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),

  "learn_filter_equivalent_2_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_3_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_4_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_5_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_6_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_7_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_8_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_9_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "learn_filter_equivalent_10_canonicalizer_equivalent_0_raw_stateless_alu_5_3".to_string(),
  "flowlets_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "flowlets_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "rcp_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),
  "rcp_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3".to_string(),

  "sampling_equivalent_1_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),

  "sampling_equivalent_2_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_3_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_4_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_5_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_6_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_7_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_8_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_9_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "sampling_equivalent_10_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1".to_string(),
  "snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),

  "snap_heavy_hitter_equivalent_2_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_3_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_4_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_5_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_6_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_7_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_8_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_9_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "snap_heavy_hitter_equivalent_10_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),

  "spam_detection_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),

  "spam_detection_equivalent_2_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_3_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_4_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_5_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_6_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_7_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_8_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_9_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "spam_detection_equivalent_10_canonicalizer_equivalent_1_pair_stateless_alu_1_1".to_string(),
  "stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "stateful_fw_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
  "stateful_fw_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string(),
     ]

}

fn test_configurations () -> Vec <Vec <String> > {

 vec![
    // simple_raw_stateless_alu_2_2
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "simple".to_string(),
        ],
    // simple_raw_stateless_alu_arith_3_4
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "3".to_string(), // Pipeline depth
          "4".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_raw_stateless_alu_arith_3_4_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],
    // simple_raw_stateless_alu_arith_2_3
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "3".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_raw_stateless_alu_arith_2_3_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],
    // simple_if_else_raw_stateless_alu_arith_rel_cond_2_2
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "3,2,0,1,4".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_if_else_raw_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],

    // simple_if_else_raw_stateless_alu_arith_rel_cond_bool2_4
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "4".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_if_else_raw_stateless_alu_arith_rel_cond_bool_2_4_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],
    // simple_sub_stateless_alu_arith_rel_cond_2_3
        
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "3".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_sub_stateless_alu_arith_rel_cond_2_3_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],
    // simple_nested_ifs_stateless_alu_arith_rel_3_2
        
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_nested_ifs_stateless_alu_arith_rel_2_2_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],
    // simple_pair_stateless_alu_arith_2_4
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "2".to_string(),
          "4".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars (always 2 for pair.alu)
          "1".to_string(), // Stateful ALUs 
          "hole_configurations/simple_pair_stateless_alu_arith_2_4_hole_cfgs.txt".to_string(), // Hole config file

          "simple".to_string(),
        ],
    vec! ["marple_new_flow".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "3".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/marple_new_flow_pred_raw_stateless_alu_arith_rel_cond_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "3".to_string(), // Pipeline width
          "1,2,3,0,4".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/marple_new_flow_sub_stateless_alu_arith_rel_cond_bool_2_3_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    // marple_new_flow_pair_stateless_alu_2_4
    vec! ["marple_new_flow".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "4".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/marple_new_flow_pair_stateless_alu_2_4_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),

        ],
    // marple_tcp_nmo_if_else_raw_stateless_alu_4_2
    vec! ["marple_tcp_nmo".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/marple_tcp_nmo_if_else_raw_stateless_alu_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo_old".to_string(),
        ],
    // marple_tcp_nmo_nested_ifs_stateless_alu_arith_rel_3_2
    vec! ["marple_tcp_nmo".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/marple_tcp_nmo_nested_ifs_stateless_alu_arith_rel_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo_old".to_string(),
        ],

    // blue_increase_pair_stateless_alu_arith_4_2
    vec! ["blue_increase".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "0,1,2,3,6,4,5,9".to_string(),
          "2".to_string(), // Num packets
          "2".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/blue_increase_pair_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file

          "blue_increase_old".to_string(),
        ],
    // learn_filter_pred_raw_stateless_alu_arith_rel_2_3
    vec! ["learn_filter".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "4,3,0,5,1,2".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/learn_filter_pred_raw_stateless_alu_arith_rel_2_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter_old".to_string(),
    ],
    // learn_filter_if_else_raw_stateless_alu_2_2
    vec! ["learn_filter".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/learn_filter_if_else_raw_stateless_alu_2_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter_old".to_string(),
        ],
    // learn_filter_sub_stateless_alu_arith_rel_cond_2_3 
    vec! ["learn_filter".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "0,1,2,3,4,5".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/learn_filter_sub_stateless_alu_arith_rel_cond_2_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter_old".to_string(),
        ],

     //sampling_sub_stateless_alu_3_3 
    vec! ["sampling".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_sub_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
           "sampling_old".to_string(),
        ],

     // sampling_pair_stateless_alu_arith_rel_3_3
    vec! ["sampling".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "1,2,0,4,3".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_pair_stateless_alu_arith_rel_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "sampling_old".to_string(),
        ],

     // rcp_if_else_raw_stateless_alu_arith_rel_3_3
    vec! ["rcp".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/rcp_if_else_raw_stateless_alu_arith_rel_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],
     // rcp_if_else_raw_stateless_alu_arith_rel_2_3
    vec! ["rcp".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/rcp_nested_ifs_stateless_alu_2_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],
     //times_two_sub_stateless_alu_arith_3_3 
    vec! ["times_two".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/times_two_sub_stateless_alu_arith_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "times_two".to_string(),
        ],
     // times_two_if_else_raw_stateless_alu_arith_rel_4_2
    vec! ["times_two".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/times_two_if_else_raw_stateless_alu_arith_rel_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "times_two".to_string(),
        ],

     // test_pred_raw_stateless_alu_arith_rel_cond_bool_3_3
    vec! ["test".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/test_pred_raw_stateless_alu_arith_rel_cond_bool_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "test".to_string(),
        ],
     // test_if_else_raw_stateless_alu_arith_rel_cond_4_4
    vec! ["test".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "4".to_string(),
          "4".to_string(),
          "0,1,2,3".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/test_if_else_raw_stateless_alu_arith_rel_cond_4_4_hole_cfgs.txt".to_string(), // Hole config file
          "test".to_string(),
        ],
     // snap_heavy_hitter_pair_stateless_alu_2_3
    vec! ["snap_heavy_hitter".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "0,1,2,3,999,997,1002,1000,4".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/snap_heavy_hitter_pair_stateless_alu_2_3_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter_old".to_string(),
        ],
     //snap_heavy_hitter_pair_stateless_alu_arith_rel_2_2 
    vec! ["snap_heavy_hitter".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3,998,4,999".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/snap_heavy_hitter_pair_stateless_alu_arith_rel_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter_old".to_string(),
        ],
     //  sampling_revised_nested_ifs_stateless_alu_arith_rel_cond_2_2
    vec! ["sampling_revised".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3,8,4,9,29,1023,5,31,33".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_revised_nested_ifs_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "sampling_revised".to_string(),
        ],
     // sampling_revised_pair_stateless_alu_arith_rel_3_3 
    vec! ["sampling_revised".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3,8,29,7,9,6,28,30,36".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_revised_pair_stateless_alu_arith_rel_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "sampling_revised".to_string(),
        ],

        // Experiment tests
        //
        
            vec! ["blue_increase_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "11,21,10,12,0,3,1,2,10,2,1".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_increase/blue_increase_2/blue_increase_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_increase".to_string(),
        ],
        

    vec! ["blue_increase_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "1,12,3,10,2,11,10,1,0,4,14,2".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_increase/blue_increase_4/blue_increase_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_increase".to_string(),
        ],


    vec! ["blue_increase_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "1,10,11,22,1,13,12,10,0,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_increase/blue_increase_6/blue_increase_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_increase".to_string(),
        ],

    vec! ["blue_increase_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "5,15,21,9,10,3,14,4,12,10,0,16,1,11,22,6,1,2,20".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_increase/blue_increase_7/blue_increase_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_increase".to_string(),
        ],


    vec! ["blue_increase_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "12,0,10,2,3,215,1,1,203,11,214,204".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_increase/blue_increase_9/blue_increase_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_increase".to_string(),
        ],
    vec! ["blue_increase_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "0,11,1,10,13,3,2,10,12,21,1".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_increase/blue_increase_10/blue_increase_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_increase".to_string(),
        ],

    vec! ["blue_decrease_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "19,10,15,18,11,1,3,1,0,12,22,2,16,10,9,17".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_1/blue_decrease_equivalent_1_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],

    vec! ["blue_decrease_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "3,12,1,2,2,0,10,11,10,1".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_2/blue_decrease_equivalent_2_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],

    vec! ["blue_decrease_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "10,0,216,10,2,3,12,205,11,1,1,4".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_3/blue_decrease_equivalent_3_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],

    vec! ["blue_decrease_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "12,11,10,10,2,0,1,1,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_4/blue_decrease_equivalent_4_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],
    vec! ["blue_decrease_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "2,12,11,1,1,2,0,10,3,10,13".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_6/blue_decrease_equivalent_6_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],
    vec! ["blue_decrease_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "2,1023,10,11,1,13,18,3,5,12,9,20,10,7,1014,0,1".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_7/blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],

    vec! ["blue_decrease_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "0,3,11,10,12,1,10,1,2,21".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_8/blue_decrease_equivalent_8_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],

    vec! ["blue_decrease_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "2,307,9,1,1,10,0,12,3,11".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/blue_decrease/blue_decrease_9/blue_decrease_equivalent_9_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_hole_cfgs.txt".to_string(), // Hole config file
          "blue_decrease".to_string(),
        ],
    vec! ["conga_equivalent_1_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_1/conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
    vec! ["conga_equivalent_2_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3,4,303,337,687".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_2/conga_equivalent_2_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
    vec! ["conga_equivalent_3_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_3/conga_equivalent_3_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
    vec! ["conga_equivalent_4_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_4/conga_equivalent_4_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
     vec! ["conga_equivalent_5_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3,4,5".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_5/conga_equivalent_5_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
   vec! ["conga_equivalent_6_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_6/conga_equivalent_6_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
    vec! ["conga_equivalent_7_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_7/conga_equivalent_7_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],

    vec! ["conga_equivalent_8_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_8/conga_equivalent_8_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],

    vec! ["conga_equivalent_9_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_9/conga_equivalent_9_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
    vec! ["conga_equivalent_10_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "5".to_string(),
          "0,1,2,3,4,303,337,687".to_string(),
          "5".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/conga/conga_10/conga_equivalent_10_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt".to_string(), // Hole config file
          "conga".to_string(),
        ],
        
    vec! ["marple_new_flow_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow1/marple_new_flow_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow2/marple_new_flow_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow3/marple_new_flow_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow4/marple_new_flow_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow5/marple_new_flow_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
        
    vec! ["marple_new_flow_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow6/marple_new_flow_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow7/marple_new_flow_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow8/marple_new_flow_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],

    vec! ["marple_new_flow_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow9/marple_new_flow_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_new_flow_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/marple_new_flow/marple_new_flow10/marple_new_flow_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_new_flow".to_string(),
        ],
    vec! ["marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo1/marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],
    vec! ["marple_tcp_nmo_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo2/marple_tcp_nmo_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo3/marple_tcp_nmo_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],
    vec! ["marple_tcp_nmo_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo4/marple_tcp_nmo_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo5/marple_tcp_nmo_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo6/marple_tcp_nmo_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo7/marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo8/marple_tcp_nmo_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo9/marple_tcp_nmo_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["marple_tcp_nmo_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/marple_tcp_nmo/marple_tcp_nmo10/marple_tcp_nmo_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt".to_string(), // Hole config file
          "marple_tcp_nmo".to_string(),
        ],

    vec! ["learn_filter_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "2,1,0,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter1/learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],
    vec! ["learn_filter_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "3,2,1,0".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter2/learn_filter_equivalent_2_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "0,3,2,1".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter3/learn_filter_equivalent_3_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "2,0,3,1".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter4/learn_filter_equivalent_4_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "1,3,2,0".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter5/learn_filter_equivalent_5_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "1,3,0,2".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter6/learn_filter_equivalent_6_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "1,3,0,2".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter7/learn_filter_equivalent_7_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "2,1,0,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter8/learn_filter_equivalent_8_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "2,1,3,0".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter9/learn_filter_equivalent_9_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],

    vec! ["learn_filter_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "5".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/learn_filter/learn_filter10/learn_filter_equivalent_10_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt".to_string(), // Hole config file
          "learn_filter".to_string(),
        ],
    vec! ["flowlets_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,7,2,5".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_1/flowlets_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,2,42,43,5,44,7,4,3,8".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_2/flowlets_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,7,2,9,10,66,4,5".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_3/flowlets_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,2,42,43,7,5,3".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_4/flowlets_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,2,42,43,4,5,7,8,10,9,301,300".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_5/flowlets_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,3,2,7,196,195,4".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_6/flowlets_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,2,42,43,44,45,50,7,5,38".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_7/flowlets_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,7,2,5".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_8/flowlets_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,7,2,9,11,3".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_9/flowlets_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],
    vec! ["flowlets_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,5,6,1,7,2,5,8,11,4,27,22".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "experiments/flowlets/flowlets_10/flowlets_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "flowlets".to_string(),
        ],

    vec! ["rcp_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3,30,31".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp1/rcp_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],
    vec! ["rcp_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 29, 30, 295, 320, 678".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp2/rcp_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 29, 30, 295, 336, 337, 872,952".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp3/rcp_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 29, 30, 192, 300, 301, 952".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp4/rcp_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3,30,31".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp5/rcp_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 30".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp6/rcp_equivalent_6_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3,30,31,192,320".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp7/rcp_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 26, 28, 29, 30, 31, 295, 952, 953".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp8/rcp_equivalent_8_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],
    vec! ["rcp_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 29, 30, 33, 34, 61".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp9/rcp_equivalent_9_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["rcp_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0, 1, 2, 3, 28, 29, 30, 31, 301, 1014".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "experiments/rcp/rcp10/rcp_equivalent_10_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt".to_string(), // Hole config file
          "rcp".to_string(),
        ],

    vec! ["sampling_equivalent_1_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "29,29,2,0,1,1,30,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling1/sampling_equivalent_1_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],


    vec! ["sampling_equivalent_2_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "30, 1,0,1, 2,29, 29, 3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling2/sampling_equivalent_2_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "2, 29, 1,0,1,29,30, 3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling3/sampling_equivalent_3_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "0,29, 1, 2, 3, 29".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling4/sampling_equivalent_4_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "1,0,29, 2,1, 29, 3,30".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling5/sampling_equivalent_5_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_6_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "1,30,0,1,29, 29, 2, 3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling6/sampling_equivalent_6_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_7_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "3, 1,0, 2,29, 29,30".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling7/sampling_equivalent_7_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_8_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "2, 1,29,4,30, 3,1,0, 29".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling8/sampling_equivalent_8_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_9_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "29,30,0,29,1, 2, 3, 1".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling9/sampling_equivalent_9_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],
    vec! ["sampling_equivalent_10_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "1".to_string(),
          "29,0,28,30, 2, 1, 3,1, 29".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/sampling/sampling10/sampling_equivalent_10_canonicalizer_equivalent_0_if_else_raw_stateless_alu_2_1_hole_cfgs.txt".to_string(), // Hole config file
          "sampling".to_string(),
        ],

    vec! ["snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter1/snap_heavy_hitter_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],
    vec! ["snap_heavy_hitter_equivalent_2_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter2/snap_heavy_hitter_equivalent_2_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_3_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,415,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter3/snap_heavy_hitter_equivalent_3_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_4_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,997,999,1000,1001".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter4/snap_heavy_hitter_equivalent_4_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_5_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,999,1000,1001".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter5/snap_heavy_hitter_equivalent_5_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_6_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter6/snap_heavy_hitter_equivalent_6_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_7_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,997,998,999,1000,1001".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter7/snap_heavy_hitter_equivalent_7_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_8_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter8/snap_heavy_hitter_equivalent_8_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_9_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter9/snap_heavy_hitter_equivalent_9_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],


    vec! ["snap_heavy_hitter_equivalent_10_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/snap_heavy_hitter/snap_heavy_hitter10/snap_heavy_hitter_equivalent_10_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "snap_heavy_hitter".to_string(),
        ],

    vec! ["spam_detection_equivalent_1_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection1/spam_detection_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_2_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection2/spam_detection_equivalent_2_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_3_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection3/spam_detection_equivalent_3_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_4_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection4/spam_detection_equivalent_4_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_5_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0, 1, 2, 3, 4, 14, 723, 998, 999, 1000,1023".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection5/spam_detection_equivalent_5_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_6_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,5,995,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection6/spam_detection_equivalent_6_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_7_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection7/spam_detection_equivalent_7_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_8_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,999,1000,1001".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection8/spam_detection_equivalent_8_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_9_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection9/spam_detection_equivalent_9_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],
    vec! ["spam_detection_equivalent_10_canonicalizer_equivalent_1".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "1".to_string(),
          "1".to_string(),
          "0,1,2,3,4,998,999,1000".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/spam_detection/spam_detection10/spam_detection_equivalent_10_canonicalizer_equivalent_1_pair_stateless_alu_1_1_hole_cfgs.txt".to_string(), // Hole config file
          "spam_detection".to_string(),
        ],


    vec! ["stateful_fw_equivalent_3_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,102,102,1,100,103".to_string(),
          "4".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/stateful_fw/stateful_fw3/stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "stateful_fw".to_string(),
        ],


    vec! ["stateful_fw_equivalent_4_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,102,102,1,101,203,2,3,103".to_string(),
          "4".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/stateful_fw/stateful_fw4/stateful_fw_equivalent_4_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "stateful_fw".to_string(),
        ],


    vec! ["stateful_fw_equivalent_5_canonicalizer_equivalent_0".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(),
          "5".to_string(),
          "0,1,2,3,102,102,1,100,103,101,3".to_string(),
          "4".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "experiments/stateful_fw/stateful_fw5/stateful_fw_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt".to_string(), // Hole config file
          "stateful_fw".to_string(),
        ],




  ]
}

