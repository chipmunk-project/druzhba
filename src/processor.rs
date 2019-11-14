use crate::phv::Phv;
use std::env;
use std::fs;
use std::process::Command;

pub struct Processor {
    pub riscv_file : String,
    pub phvs : Vec<Phv<i32>>,
    pub state : Vec<i32>,
}

impl Processor {
    pub fn new () -> Processor {
        Processor {
            riscv_file : String::from(""),
            phvs : Vec::new(),
            state: vec![0,1],
        }
    }

    pub fn with_riscv_file (t_riscv_file : String) 
        -> Processor {
         Processor {
            riscv_file : t_riscv_file,
            phvs : Vec::new(),
            state : vec![0;1],
        }
       
    }
    pub fn add_phv (&mut self, input_phv : Phv<i32>)
    {
       self.phvs.push(input_phv); 
    }
    
    pub fn execute_program (&mut self) {
       let current_phv = self.phvs.remove(0);
       let mut binary_file = self.riscv_file.clone();
       binary_file.truncate(binary_file.len() - 2);

       let complete_riscv_file : String = format!("{}{}.s", 
                                                  binary_file, 
                                                  "_complete");

       self.generate_state_variables(&complete_riscv_file);
/*       let riscv_file_contents : String = 
            fs::read_to_string(self.riscv_file.clone())
            .expect("Error: RISCV file could not be found. Double check the that the file path is correct");*/
       let riscv_file_contents : String = fs::read_to_string(complete_riscv_file.clone())
            .expect("Error: RISCV file could not be found. Double check the that the file path is correct");


       fs::write(complete_riscv_file.clone(),
                 format!("{}{}",
                         riscv_file_contents,
                         self.generate_main_function(current_phv)));

       Command::new("riscv64-unknown-linux-gnu-gcc")
                .arg("-static")
                .arg(&complete_riscv_file)
                .arg("-o")
                .arg(&binary_file)
                .output()
                .expect("Could not run cross compiler");


       let output = Command::new("spike")
                .arg("pk")
                .arg(&binary_file)
                .output()
                .expect("Could not run spike");
       println!("{}", String::from_utf8_lossy(&output.stdout));
       self.process_results();

        Command::new("rm")
                .arg(&"state.txt")
                .output()
                .expect("Could not remove state.txt");
   
       Command::new("rm")
                .arg(&binary_file)
                .output()
                .expect("Could not remove binary");
       Command::new("rm")
                .arg(&complete_riscv_file)
                .output()
                .expect("Could not complete RISCV file");
    }
    fn process_results (&mut self) {
       let results_contents : String = fs::read_to_string("results.txt").expect("Error: results.txt could not be opened");
       let lines : Vec<String> = results_contents.split("\n")
                                                 .map(|s| s.to_string())
                                                 .collect(); 
       println!("Result: {}", lines[0]);
       let state_results : Vec <&str>= lines[1].split(",").collect();
       for i in 0..state_results.len(){
         self.state[i] = match state_results[i].parse::<i32>() {
           Ok (t_sv) => t_sv,
           Err (_)   => panic!("Error: unable to unwrap state variable from results.txt"),

         };
       }
       println!("State result: {:?}\n", self.state);
       
    }
    fn generate_state_variables (&self, complete_riscv_file : &str) {
       let riscv_file_contents : String = 
            fs::read_to_string(self.riscv_file.clone())
            .expect("Error: RISCV file could not be found. Double check the that the file path is correct");
       let mut new_riscv_file_contents : String = String::from("");
       let lines : Vec<String> = riscv_file_contents.split("\n")
                                                    .map(|s| s.to_string())
                                                    .collect(); 
       println!("Using previous state {:?}", self.state);
       let mut in_state : bool = false;
       for line in lines.iter() {
           if line.contains(".ident") {
              new_riscv_file_contents.push_str("  .align 1\n");
              break;
           }  
           if in_state {
               for var in self.state.iter() {
                   //println!("Writing state var {}", var);
                   new_riscv_file_contents.push_str(&format!("  .word {}\n", var));
               }
               in_state = false;
           }
           else if line.contains(".bss") {
               new_riscv_file_contents.push_str(".data\n");
           }
           else {
               new_riscv_file_contents.push_str(&line);
               new_riscv_file_contents.push_str("\n");
           }     
           if line.contains("state:") {
               in_state = true;
           } 
       }
      fs::write(complete_riscv_file.clone(),
                new_riscv_file_contents);

    }
    fn generate_main_function (&self, input_phv : Phv<i32>) 
        -> String {
        
        let mut riscv_file_contents : String = fs::read_to_string(&self.riscv_file)
            .expect ("Error: Hole configs file could not be found");
        let mut main_function : String = String::from("  .globl main\n  .type main, @function\nmain:\n");

        // TODO: stack_frame_offset should be dynamic
        // depending on the number of PHV containers..
        // -32 is good for up to 4 PHV containers
        //
        // stack_frame_offset is the number of bytes the
        // stack frame will take. sp is the stack pointer
        // register. s0 contains the address of the beginning
        // of the stack frame. ra is the return address
        let stack_frame_offset = -32;
        main_function.push_str(&format!("  addi  sp,sp,{}\n",
                                        stack_frame_offset));
        main_function.push_str(&format!("  sd  ra,{}(sp)\n",
                                        stack_frame_offset*-1 - 8));
        main_function.push_str(&format!("  sd  s0,{}(sp)\n",
                                        stack_frame_offset*-1 - 16));
        main_function.push_str(&format!("  addi  s0,sp,{}\n",
                                        stack_frame_offset*-1));

        if -24 + 4 * input_phv.get_num_phv_containers() < stack_frame_offset{
            panic!("Error: stack overflow may occur. Ensure that stack_frame_offset is large enough for all values");
        }

        // Load PHV container values into RISCV.
        // sw is used because i32 values are used only
        //
        // -24 is used because slots -8 and -16 are taken.
        // Currently only 4 PHV containers supported
        let function_argument_instructions : &str = 
            &match input_phv.get_num_phv_containers() {
            1 => format!("{}{}{}",
                         self.generate_load_immediate(&input_phv, 0),
                         self.generate_store(&input_phv,stack_frame_offset + 8),
                         self.generate_argument_loads(1)),

            2 => format!("{}{}{}{}{}",
                          self.generate_load_immediate(&input_phv, 0),
                          self.generate_store(&input_phv,stack_frame_offset + 8),
                          self.generate_load_immediate(&input_phv, 1),
                          self.generate_store(&input_phv, stack_frame_offset + 12),
                          self.generate_argument_loads(2)),
            3 => format!("{}{}{}{}{}{}{}",
                          self.generate_load_immediate(&input_phv, 0),
                          self.generate_store(&input_phv,stack_frame_offset),
                          self.generate_load_immediate(&input_phv, 1),
                          self.generate_store(&input_phv, stack_frame_offset + 4),
                          self.generate_load_immediate(&input_phv, 2),
                          self.generate_store(&input_phv, stack_frame_offset + 8),
                          self.generate_argument_loads(3)),
            4 => format!("{}{}{}{}{}{}{}{}{}",
                          self.generate_load_immediate(&input_phv, 0),
                          self.generate_store(&input_phv,stack_frame_offset),
                          self.generate_load_immediate(&input_phv, 1),
                          self.generate_store(&input_phv, stack_frame_offset + 4),
                          self.generate_load_immediate(&input_phv, 2),
                          self.generate_store(&input_phv, stack_frame_offset + 8),
                          self.generate_load_immediate(&input_phv, 3),
                          self.generate_store(&input_phv, stack_frame_offset + 12),
                          self.generate_argument_loads(4)),

            _ => panic!("Error: Only a maximum of 4 PHV containers can be handled for RISCV simulations"),
        };
            
        
        main_function.push_str(function_argument_instructions);
        let split_path : Vec<String> = self.riscv_file.clone()
                                           .split("/")
                                           .map(|s| s.to_string())
                                           .collect();
        let mut function_name : String = split_path[split_path.len() -1 ].clone();
        function_name.truncate(function_name.len() - 2);
 
        main_function.push_str(&format!("  call {}\n", function_name));
        main_function.push_str(&format!("  ld  ra,{}(sp)\n",
                                        stack_frame_offset*-1 - 8));
        main_function.push_str(&format!("  ld  s0,{}(sp)\n",
                                       stack_frame_offset*-1 - 16));
        main_function.push_str(&format!("  addi  sp,sp,{}\n",
                                        stack_frame_offset*-1));
        main_function.push_str("  jr ra\n  .size main, .-main\n  .ident \"GCC: (GNU) 9.2.0\"\n");
        main_function

    }
    // Generates the instructions to store immediates 
    // (i.e. constants) into the a5 register. This is 
    // the value stored inside of a PHV container
    fn generate_load_immediate (&self, 
                                phv : &Phv<i32>,
                                index : i32) -> String {
        format!("  li  a5,{}\n", phv[index].get_value())
    }
    // Generates the isntructions to store the instructions
    // from the a5 register to the stack
    fn generate_store (&self,
                       phv : &Phv<i32>,
                       offset : i32) -> String {
        format!("  sw  a5,{}(s0)\n", offset)
    }
    // Generates the instructions to load function arguments
    // into registers. The function that is called will
    // pull the values from these registers and load them
    // into its newly created stack frame
    fn generate_argument_loads (&self,
                                num_args : i32) -> String {

        match num_args {
            1 => format!("  lw  a0,-24(s0)\n"),
            2 => format!("  ld  a0,-24(s0)\n"),
            3 => format!("  ld  a0,-32(s0)\n  ld  a1,-24(s0)\n"),
            4 => format!("  ld  a0,-32(s0)\n  ld  a1,-24(s0)\n"), 
            _ => panic!("Error: Only a maximum of 4 PHV containers can be handled for RISCV simulations"),
        }
    }
                                
}
