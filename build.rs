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

  let test_case_names : Vec <String> = 
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
      ];
  let dgen_data : Vec <Vec <String> > = vec![
    // simple_raw_stateless_alu_2_2
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs (Depends on modified variables in the spec)
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt".to_string() // Hole config file
        ],
    // simple_raw_stateless_alu_arith_3_4
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "3".to_string(), // Pipeline depth
          "4".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_raw_stateless_alu_arith_3_4_hole_cfgs.txt".to_string() // Hole config file
        ],
    // simple_raw_stateless_alu_arith_2_3
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "3".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_raw_stateless_alu_arith_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],
    // simple_if_else_raw_stateless_alu_arith_rel_cond_2_2
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "3,2,0,1,4".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_if_else_raw_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt".to_string() // Hole config file
        ],

    // simple_if_else_raw_stateless_alu_arith_rel_cond_bool2_4
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "4".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_if_else_raw_stateless_alu_arith_rel_cond_bool_2_4_hole_cfgs.txt".to_string() // Hole config file
        ],
    // simple_sub_stateless_alu_arith_rel_cond_2_3
        
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "3".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_sub_stateless_alu_arith_rel_cond_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],
    // simple_nested_ifs_stateless_alu_arith_rel_3_2
        
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/simple_nested_ifs_stateless_alu_arith_rel_2_2_hole_cfgs.txt".to_string() // Hole config file
        ],
    // simple_pair_stateless_alu_arith_2_4
    vec! ["simple".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "2".to_string(),
          "4".to_string(),
          "1".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars (always 2 for pair.alu)
          "1".to_string(), // Stateful ALUs 
          "hole_configurations/simple_pair_stateless_alu_arith_2_4_hole_cfgs.txt".to_string() // Hole config file
        ],
    vec! ["marple_new_flow".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "3".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/marple_new_flow_pred_raw_stateless_alu_arith_rel_cond_3_2_hole_cfgs.txt".to_string() // Hole config file
        ],
    vec! ["marple_new_flow".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "3".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "1,2,3,0,4".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/marple_new_flow_sub_stateless_alu_arith_rel_cond_bool_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],
    // marple_new_flow_pair_stateless_alu_2_4
    vec! ["marple_new_flow".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(), // Pipeline depth
          "4".to_string(), // Pipeline width
          "1".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/marple_new_flow_pair_stateless_alu_2_4_hole_cfgs.txt".to_string() // Hole config file
        ],
    // marple_tcp_nmo_if_else_raw_stateless_alu_4_2
    vec! ["marple_tcp_nmo".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "4".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "2".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/marple_tcp_nmo_if_else_raw_stateless_alu_4_2_hole_cfgs.txt".to_string() // Hole config file
        ],
    // marple_tcp_nmo_nested_ifs_stateless_alu_arith_rel_3_2
    vec! ["marple_tcp_nmo".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(), // Pipeline depth
          "2".to_string(), // Pipeline width
          "2".to_string(), // Stateful ALUs
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/marple_tcp_nmo_nested_ifs_stateless_alu_arith_rel_3_2_hole_cfgs.txt".to_string() // Hole config file
        ],

    // blue_increase_pair_stateless_alu_arith_4_2
    vec! ["blue_increase".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "2".to_string(),
          "0,1,2,3,6,4,5,9".to_string(),
          "2".to_string(), // Num packets
          "2".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/blue_increase_pair_stateless_alu_arith_4_2_hole_cfgs.txt".to_string() // Hole config file
        ],
    // learn_filter_pred_raw_stateless_alu_arith_rel_2_3
    vec! ["learn_filter".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "3".to_string(),
          "4,3,0,5,1,2".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/learn_filter_pred_raw_stateless_alu_arith_rel_2_3_hole_cfgs.txt".to_string() // Hole config file
    ],
    // learn_filter_if_else_raw_stateless_alu_2_2
    vec! ["learn_filter".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/learn_filter_if_else_raw_stateless_alu_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],
    // learn_filter_sub_stateless_alu_arith_rel_cond_2_3 
    vec! ["learn_filter".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3,4,5".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/learn_filter_sub_stateless_alu_arith_rel_cond_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],

     //sampling_sub_stateless_alu_3_3 
    vec! ["sampling".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "1".to_string(),
          "0,1,2,3".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_sub_stateless_alu_3_3_hole_cfgs.txt".to_string() // Hole config file
        ],

     // sampling_pair_stateless_alu_arith_rel_3_3
    vec! ["sampling".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "1".to_string(),
          "1,2,0,4,3".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_pair_stateless_alu_arith_rel_3_3_hole_cfgs.txt".to_string() // Hole config file
        ],

     // rcp_if_else_raw_stateless_alu_arith_rel_3_3
    vec! ["rcp".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/rcp_if_else_raw_stateless_alu_arith_rel_3_3_hole_cfgs.txt".to_string() // Hole config file
        ],
     // rcp_if_else_raw_stateless_alu_arith_rel_2_3
    vec! ["rcp".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "3".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "3".to_string(), // Stateful ALUs
          "hole_configurations/rcp_nested_ifs_stateless_alu_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],
     //times_two_sub_stateless_alu_arith_3_3 
    vec! ["times_two".to_string(),
          "example_alus/stateful_alus/sub.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "1".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/times_two_sub_stateless_alu_arith_3_3_hole_cfgs.txt".to_string() // Hole config file
        ],
     // times_two_if_else_raw_stateless_alu_arith_rel_4_2
    vec! ["times_two".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "4".to_string(),
          "2".to_string(),
          "1".to_string(),
          "0,1,2,3".to_string(),
          "2".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/times_two_if_else_raw_stateless_alu_arith_rel_4_2_hole_cfgs.txt".to_string() // Hole config file
        ],

     // test_pred_raw_stateless_alu_arith_rel_cond_bool_3_3
    vec! ["test".to_string(),
          "example_alus/stateful_alus/pred_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/test_pred_raw_stateless_alu_arith_rel_cond_bool_3_3_hole_cfgs.txt".to_string() // Hole config file
        ],
     // test_if_else_raw_stateless_alu_arith_rel_cond_4_4
    vec! ["test".to_string(),
          "example_alus/stateful_alus/if_else_raw.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "4".to_string(),
          "4".to_string(),
          "2".to_string(),
          "0,1,2,3".to_string(),
          "3".to_string(), // Num packets
          "1".to_string(), // State vars
          "2".to_string(), // Stateful ALUs
          "hole_configurations/test_if_else_raw_stateless_alu_arith_rel_cond_4_4_hole_cfgs.txt".to_string() // Hole config file
        ],
     // snap_heavy_hitter_pair_stateless_alu_2_3
    vec! ["snap_heavy_hitter".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu.alu".to_string(),
          "2".to_string(),
          "3".to_string(),
          "1".to_string(),
          "0,1,2,3,999,997,1002,1000,4".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/snap_heavy_hitter_pair_stateless_alu_2_3_hole_cfgs.txt".to_string() // Hole config file
        ],
     //snap_heavy_hitter_pair_stateless_alu_arith_rel_2_2 
    vec! ["snap_heavy_hitter".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "1".to_string(),
          "0,1,2,3,998,4,999".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/snap_heavy_hitter_pair_stateless_alu_arith_rel_2_2_hole_cfgs.txt".to_string() // Hole config file
        ],
     //  sampling_revised_nested_ifs_stateless_alu_arith_rel_cond_2_2
    vec! ["sampling_revised".to_string(),
          "example_alus/stateful_alus/nested_ifs.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu".to_string(),
          "2".to_string(),
          "2".to_string(),
          "1".to_string(),
          "0,1,2,3,8,4,9,29,1023,5,31,33".to_string(),
          "1".to_string(), // Num packets
          "1".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_revised_nested_ifs_stateless_alu_arith_rel_cond_2_2_hole_cfgs.txt".to_string() // Hole config file
        ],
     // sampling_revised_pair_stateless_alu_arith_rel_3_3 
    vec! ["sampling_revised".to_string(),
          "example_alus/stateful_alus/pair.alu".to_string(),
          "example_alus/stateless_alus/stateless_alu_arith_rel.alu".to_string(),
          "3".to_string(),
          "3".to_string(),
          "1".to_string(),
          "0,1,2,3,8,29,7,9,6,28,30,36".to_string(),
          "1".to_string(), // Num packets
          "2".to_string(), // State vars
          "1".to_string(), // Stateful ALUs
          "hole_configurations/sampling_revised_pair_stateless_alu_arith_rel_3_3_hole_cfgs.txt".to_string() // Hole config file
        ],





  ];
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
                 .arg(&arg[5]) // Stateful ALUs
                 .arg(&arg[6]) // constant vec
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
                      num_packets = dgen_data[7],
                      num_containers = dgen_data[4], 
                      num_state_vars = dgen_data[8],
                      num_stateful_alus = dgen_data[9],
                      hole_configurations = dgen_data[10],
                      prog_to_run_file = test_name,
                      test_function = format!("test_{}",dgen_data[0])
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
