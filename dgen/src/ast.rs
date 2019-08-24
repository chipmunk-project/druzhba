use std::fmt;

use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
  // Need a rw lock because Rust does not like
  // aliasing + mutation at the same time. The
  // rw lock provides the necessary synchronization
  static ref HELPER_STRING: RwLock<String> = 
    RwLock::new(String::from(""));
  static ref STATE_VAR_MAP : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref HOLE_VAR_MAP : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref PHV_CONTAINER_MAP : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref PIPELINE_STAGE : RwLock <i32> = 
      RwLock::new(0);
  static ref ALU_NUMBER : RwLock <i32> = 
      RwLock::new(0);
  static ref NAME : RwLock <String> =
      RwLock::new(String::from(""));
  static ref FUNC_COUNT : RwLock <HashMap <String, i32> >=
      RwLock::new (HashMap::new());
}

pub enum Alu {
  Program ( Option<Box<OptHeader>>, Box <Header>, Box <Stmt>),
}
impl fmt::Display for Alu {
  fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Alu::Program (opt_header, header, stmt) => {
        { // Write lock in its own scope to avoid deadlock
          let mut tmp = HELPER_STRING.write().unwrap();
          *tmp = String::from("");
          STATE_VAR_MAP.write().unwrap().clear();
          HOLE_VAR_MAP.write().unwrap().clear();
          PHV_CONTAINER_MAP.write().unwrap().clear();
          *PIPELINE_STAGE.write().unwrap() = 0;
          NAME.write().unwrap().clear();
          FUNC_COUNT.write().unwrap().clear();
        }
        
        match opt_header {
          Some (h) => write!(f, "{}", h),
          None     => write!(f, ""),
        }.expect ("Error: issue with match statement on OptHeader");
        // The function inside of the initialize ALU function that
        // will be returned
        let inner_header : String = String::from ("    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> Vec <i32>{\n");
        let body : String = String::from(&format!("{}{}", header, stmt));
        let state_var_length = STATE_VAR_MAP.read().unwrap().len();
        let mut end : String = String::from("");
        
        if state_var_length == 0 {
          end.push_str("    };\n   Box::new(alu)\n}\n"); 
        }
        else {
          end.push_str("    old_state\n    };\n    Box::new(alu)\n}\n");
        }

        // Function name to initialize ALU function
        let init_name : String = 
            format! ("init_{}_{}_{}_{}", 
                &NAME.read().unwrap().to_string(),
                match FUNC_COUNT.read().unwrap()["state"] {
                  0 => "stateful_alu",
                  1 => "stateless_alu",
                  _ => panic!("Error: invalid state indicator"),
                },
                *PIPELINE_STAGE.read().unwrap(),
                *ALU_NUMBER.read().unwrap() );


        let mut outer_header : String = String::from("pub fn ");
        outer_header.push_str (&init_name);
        outer_header.push_str ("(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> Vec <i32> >{\n");
        
        write!(f, "{}{}{}{}", outer_header, inner_header, body, end)
      },
    }
  }
}
pub enum OptHeader {
  Data (String,  // Sketch name
        i32, // Pipeline stage
        i32) // ALU number
}

impl fmt::Display for OptHeader {

  fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
    match *&self {
      OptHeader::Data (name, stage, alu_num) => {
          
        *NAME.write().unwrap() = name.to_string();
        *PIPELINE_STAGE.write().unwrap() = *stage;       
        *ALU_NUMBER.write().unwrap() = *alu_num;
        write!(f, "")
     }
    }
  }
}
pub enum Header {
  InputData (String, // "stateful" or "stateless"
             Vec<String>, // State variables
             Vec<String>, // Hole variables
             Vec<String>), // Phv containers

}
impl fmt::Display for Header {
  fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
    match &*self{
      Header::InputData (s, v1, v2, v3) => {
        if s == "stateless" {
          FUNC_COUNT.write().unwrap().insert("state".to_string(), 1);
        }
        else if s == "stateful"{

          FUNC_COUNT.write().unwrap().insert("state".to_string(), 0);
        }
        
        if s == "stateless" && v1.len() > 0 {
          panic! ("State variables given to statelss ALU");
        }
        if v1.len() > 0 {
          for i in 0..v1.len(){
            STATE_VAR_MAP.write().unwrap().insert( v1[i].clone(), i as i32);
          }
        }
        if v2.len() > 0{
          for i in 0..v2.len(){
            HOLE_VAR_MAP.write().unwrap().insert( v2[i].clone(), i as i32);
          }
        }
        if v3.len() > 0 {
          for i in 0..v3.len() {
            PHV_CONTAINER_MAP.write().unwrap().insert(v3[i].clone(), 
                                                      i as i32);
            FUNC_COUNT.write().unwrap().insert (v3[i].clone(), 0);
          }
        }
        if v1.len() > 0 {
          write!(f, "    let old_state : Vec<i32> = state_vec.clone();\n")
        }
        else {
          write!(f, "")
        }
      },
    }
  }
}
pub enum Stmt {
  Return (Box <Expr>), 
  If (Box<Expr>, // If expr
    Vec <Box<Stmt>>, // If body
    Vec<(Box<Expr>, Vec <Box<Stmt>>)>, // Pairs of elif expr along
                                       // with expr statements
    Option<Vec<Box<Stmt>>>), // Else body
  Assign (Box<Expr>, Box<Expr>), // Variable assigned to an expr
}

impl fmt::Display for Stmt {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Need to borrow self so we don't take ownership
    // of data in tuple
    match &*self{
      Stmt::Return (e) => 
          write!(f, "        vec![({}) as i32]\n", e),

      // Generates if statement
      Stmt::If (e1, if_block, elif_block, else_block) => {
          let mut if_body : String = String::from("");
          let mut elif_bodies : String = String::from("");

          let opt_inequality : String = 
              match FUNC_COUNT.read().unwrap()["state"]{
                  0 => String::from("!= 0"),
                  1 => String::from(""),
                  _ => panic! ("Error: incorrect code given to state indicator"), 
          };
          if_body.push_str (&format!("        if {} {}{{\n", e1, 
                                     opt_inequality));
          for stmt in if_block{
            if_body.push_str (&stmt.to_string());
          }
          if_body.push_str(&"        }\n".to_string());
          for (expr, stmts) in elif_block{
            elif_bodies.push_str (&"        else if ".to_string());
            elif_bodies.push_str(&expr.to_string());
            elif_bodies.push_str(&format!("{}{{\n", opt_inequality));
            for stmt in stmts{
              elif_bodies.push_str(&stmt.to_string());
            }
            elif_bodies.push_str(&"        }\n".to_string());
          }
          let else_body : String= match else_block {
            Some(else_block_stmts) => {
              let mut tmp_str : String = String::from("        else{\n");
              for stmt in else_block_stmts{
                tmp_str.push_str(&stmt.to_string());
              }
              tmp_str.push_str(&"        }\n".to_string());
              tmp_str
            }
            _         => String::from("")
          };
   
          
          write!(f, "{}{}{}\n",
                 if_body, elif_bodies, else_body)
      },
      // TODO: Possibly check if the var name is an invalid expr.
      // Otherwise, just wait until rustc catches it
      Stmt::Assign (s, e) => write!(f, "        {} = {};\n", s, e),
    }
  }
}

pub enum Expr {
  // Captures parenthesis
  ExprWithParen (Box <Expr>),
  Num (i32),
  Var (String),
  Op (Box <Expr>, Opcode, Box<Expr> ),
//  Mux3WithNum (Box <Expr>, Box<Expr>, Box<Expr> ),
  Mux3 (Box <Expr>, Box <Expr>, Box <Expr> ),
  Mux2 (Box <Expr>, Box <Expr>),
  Opt (Box <Expr>),
  ArithOp (Box<Expr>, Box<Expr>),
  Relop (Box<Expr>, Box<Expr>),
  Constant,
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    // Updates the FUNC_COUNT HashMap to keep track of how many
    // of each helper function is in the ALU. These counters
    // are used to create the hole variable names and function
    // values
    let update_func_count = | key : String | {

      let mut func_count_ref = FUNC_COUNT.write().unwrap();
      match func_count_ref.get(&key){
        Some (&count) => func_count_ref.insert (key.clone(), count + 1),
        _             => func_count_ref.insert (key.clone(), 0),
      };
    };
    // Generates the hole name for a specific ALU helper function
    // or for a hole variable
    let generate_hole_name = 
        | mut helper_name : String| -> String {
            
      update_func_count (helper_name.clone());
      let helper_num : i32 = FUNC_COUNT.read().unwrap() [&helper_name];
      let pipeline : i32 = *PIPELINE_STAGE.read().unwrap();
      let alu : i32 = *ALU_NUMBER.read().unwrap();
      let name : String = NAME.read().unwrap().clone();
      if helper_name == "immediate_operand" {
        helper_name = "immediate".to_string();
      }
      match FUNC_COUNT.read().unwrap()["state"] {
        0 => format!("{}_stateful_alu_{}_{}_{}_{}_global", name, pipeline, alu, helper_name, helper_num),
        // TODO: Support a global format in case a stateless ALU
        // has helper functions
        1 => format!("{}_stateless_alu_{}_{}_{}", name, pipeline, alu, helper_name),
        _ => panic! ("Error: incorrect code given to state indicator"),
      }
    }; 
    // Generates the function name for a helper function
    let generate_helper_name =
        | helper_name : String| -> String {
  
        let helper_num : i32 = FUNC_COUNT.read().unwrap() [&helper_name];
        let pipeline : i32 = *PIPELINE_STAGE.read().unwrap();
        let alu : i32 = *ALU_NUMBER.read().unwrap();
        let name : String = NAME.read().unwrap().clone();

        match FUNC_COUNT.read().unwrap()["state"] {
          0 => format!("{}_stateful_alu_{}_{}_{}_{}", name, pipeline, alu, helper_name, helper_num),
          // TODO: Stateless ALU specs should not be creating any
          // helpers like in Chipmunk so either change this to 
          // support a scenario in which that happens or panic
          1 => format!("{}_stateless_alu_{}_{}_{}", name, pipeline, alu, helper_name),
          _ => panic! ("Error: incorrect code given to state indicator"),
          
      }
    };

    // Takes in an identifier and checks whether it is valid variable
    // and generates code to access that value. Since state variables
    // are in a vector, it figures out which vector index that
    // variable corresponds to. The same is done with phv_containers.
    // For hole variables, make a call to generate_hole_name
    let find_var_name = | variable : String| -> String {
      match STATE_VAR_MAP.read().unwrap().get(&variable){
        Some (ind1) => format!("state_vec[{}]",ind1),
        _           => {
          match PHV_CONTAINER_MAP.read().unwrap().get(&variable){
            Some (ind2) => format!("phv_containers[{}].get_value()",ind2),
            _           => {
              match HOLE_VAR_MAP.read().unwrap().get(&variable){
                Some (_ind3) => format!("hole_vars[\"{}\"]", generate_hole_name (variable.clone())),
                
                _            => panic! ("Error: variable '{}' is undefined", variable),

              }
            }, 
          }
        },
      }
    };
    match &*self{
      Expr::ExprWithParen (e) => 
          write!(f, "({})",e),
      Expr::Num (n)=> write! (f, "{}",n),
      Expr::Var (v)=> write! (f, "{}", find_var_name(v.to_string())),
      Expr::Op (e1, op, e2)=> 
          write! (f, "{}{}{}", e1, op, e2),
       
      // Generates the calls to the helper functions and the 
      // helper functions themselves
      Expr::Mux3 (e1, e2, e3) => { 
          let mux3_hole_name : String = 
              generate_hole_name ("Mux3".to_string());
          let mux3_name : String =
              generate_helper_name ("Mux3".to_string());
          generate_mux3(mux3_name.clone());
          write! (f, "{}({}, {}, {}, hole_vars[\"{}\"])",
                  mux3_name,
                  e1, e2, e3, mux3_hole_name)
      },
      Expr::Mux2 (e1, e2) => {
          let mux2_hole_name : String = 
              generate_hole_name ("Mux2".to_string());

          let mux2_name : String = 
              generate_helper_name ("Mux2".to_string());
          generate_mux2(mux2_name.clone());
          write! (f, "{}({}, {}, hole_vars[\"{}\"])", 
                  mux2_name,
                  e1, e2, mux2_hole_name)
      },
      Expr::Opt (e) => {
          let opt_hole_name : String = 
              generate_hole_name ("Opt".to_string());
          let opt_name : String = 
              generate_helper_name ("Opt".to_string());
          generate_opt(opt_name.clone());
          write! (f, "{}({}, hole_vars[\"{}\"])", 
                  opt_name,
                  e, opt_hole_name)
      },
      Expr::ArithOp (e1, e2) => {
          let arith_op_hole_name : String = 
              generate_hole_name ("arith_op".to_string());

        let arith_op_name : String = 
            generate_helper_name ("arith_op".to_string());
        generate_arith_op(arith_op_name.clone());
        write! (f, "{} ({}, {}, hole_vars[\"{}\"])", 
                arith_op_name,
                e1, e2, arith_op_hole_name)
      },
      Expr::Relop (e1, e2) => {
        let rel_op_hole_name : String = 
            generate_hole_name ("rel_op".to_string());

        let rel_op_name : String = 
            generate_helper_name ("rel_op".to_string());
        generate_rel_op(rel_op_name.clone());
        write! (f, "{}({}, {}, hole_vars[\"{}\"])", 
                rel_op_name,
                e1, e2, rel_op_hole_name)
      },
      Expr::Constant => {
        let constant_hole_name : String = 
            generate_hole_name ("const".to_string());

        let constant_name : String = 
            generate_helper_name ("const".to_string());
            generate_constant(constant_name.clone());
            write!(f, "{}(hole_vars[\"{}\"])", 
                   constant_name,
                   constant_hole_name)
      },
    }
  }
}
pub enum Opcode {
  Mul,
  Div,
  Add,
  Sub,
  Equal,
  Greater,
  Less,
  GreaterOrEqual,
  LessOrEqual,
  NotEqual,
  Or,
  And,
}

impl fmt::Display for Opcode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self{
      Opcode::Mul            => write! (f, "*"),
      Opcode::Div            => write! (f, "/"),
      Opcode::Add            => write! (f, "+"),
      Opcode::Sub            => write! (f, "-"),
      Opcode::Equal          => write! (f, "=="),
      Opcode::Greater        => write!(f, ">"),
      Opcode::Less           => write!(f, "<"),
      Opcode::GreaterOrEqual => write! (f, ">="),
      Opcode::LessOrEqual    => write! (f, "<="),       
      Opcode::NotEqual       => write! (f, "!="),
      Opcode::Or             => write!(f, "||"),  
      Opcode::And            => write!(f, "&&"), 
    }
  }
}

// Generates helper code for ALU function by 
// using static variable HELPER_STRING. 

pub fn get_helper_string() -> String
{
  HELPER_STRING.read().unwrap().clone()
}

fn generate_mux2 (mux2_name : String) 
{
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32, ctrl : i32) -> i32{{\n", 
              mux2_name);
  let if_ret : String = String::from
      ("  if ctrl == 0 {\n    op1\n  }\n");
  let else_ret : String = String::from
      ("  else {\n  op2\n  }\n}\n");

  let mut mux2_fn : String = String::from("");
  mux2_fn.push_str (&fn_header);
  mux2_fn.push_str(&if_ret);
  mux2_fn.push_str(&else_ret);

  HELPER_STRING.write().unwrap().push_str (&mux2_fn);
}

fn generate_mux3 (mux3_name : String)
{
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{{\n", 
              mux3_name);
  let if_ret : String = String::from
      ("  if ctrl == 0 {\n    op1\n  }\n");
  let else_if_ret : String = String::from
      ("  else if ctrl == 1 {\n    op2\n  }\n");
  let else_ret : String = String::from
      ("  else {\n  op2\n  }\n}\n");

  let mux3_fn : String= format! ("{}{}{}{}", fn_header, 
                                 if_ret, else_if_ret, else_ret);

  HELPER_STRING.write().unwrap().push_str (&mux3_fn);
}
fn generate_rel_op (rel_op_name : String)
{
  let fn_header : String = 
      format!("fn {} (op1 : i32, op2 : i32, opcode : i32) -> i32{{\n",
              rel_op_name);
  
  let if_ret : String = String::from
      ("  if opcode == 0 {\n    (op1 != op2) as i32\n  }\n");
  let elif_ret_1 : String = String::from
      ("  else if opcode == 1{\n    (op1 < op2) as i32\n  }\n");
  let elif_ret_2 : String = String::from
      ("  else if opcode == 2{\n    (op1 > op2) as i32\n  }\n");
  let else_ret : String = String::from
      ("  else{\n    (op1 == op2) as i32\n  }\n}\n");
  let mut rel_op_fn : String = String::from("");
  rel_op_fn.push_str (&fn_header);
  rel_op_fn.push_str (&if_ret);
  rel_op_fn.push_str (&elif_ret_1);
  rel_op_fn.push_str (&elif_ret_2);
  rel_op_fn.push_str (&else_ret);
  
  HELPER_STRING.write().unwrap().push_str(&rel_op_fn);
} 
fn generate_arith_op (arith_op_name : String)
{
  let fn_header : String =
      format!("fn {} (op1 : i32, op2 : i32, opcode : i32) -> i32{{\n",
              arith_op_name);
  let if_ret : String = String::from
      ("  if opcode == 0 {\n    op1 + op2\n  }\n");
  let else_ret : String = String::from 
      ("  else {\n  op1 - op2\n  }\n}\n");
  let mut arith_op_fn : String = String::from("");
  arith_op_fn.push_str (&fn_header);
  arith_op_fn.push_str (&if_ret);
  arith_op_fn.push_str (&else_ret);
  HELPER_STRING.write().unwrap().push_str (&arith_op_fn);
} 
fn generate_constant(constant_name : String)
{
  let fn_header : String = 
      format! ("fn {} (constant : i32) -> i32 {{\n",
               constant_name);
  let ret : String = String::from
      ("  constant\n}\n");
  let mut const_fn : String = String::from("");
  const_fn.push_str (&fn_header);
  const_fn.push_str (&ret);
  HELPER_STRING.write().unwrap().push_str (&const_fn);

}
fn generate_opt(opt_name : String)
{
  let fn_header : String =
      format! ("fn {} (op : i32, enable : i32) -> i32 {{\n",
               opt_name);
  let if_ret : String = String::from
      ("  if enable != 0 {\n    0\n  }\n");
  let else_ret : String = String::from
      ("else{\n  op\n  }\n}\n");
  let mut opt_fn = String::from("");
  opt_fn.push_str (&fn_header);
  opt_fn.push_str (&if_ret);
  opt_fn.push_str (&else_ret);
  HELPER_STRING.write().unwrap().push_str (&opt_fn);
}

