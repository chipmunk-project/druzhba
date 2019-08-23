use std::fmt;

use std::collections::HashMap;
use std::sync::RwLock;
lazy_static! {
  // Need a rw lock because Rust does not like 
  // aliasing + mutation at the same time. The
  // rw lock provides necessary synchronization
  static ref HELPER_STRING : RwLock <String> = 
      RwLock::new(String::from(""));
}
pub enum Alu {
  Program ( Option<Box<OptHeader>>, Box <Header>, Box <Stmt>),
}

pub enum OptHeader {
  Data (String,  // Sketch name
        i32, // Pipeline stage
        i32) // ALU number
}

pub enum Header {
  InputData (String, // "stateful" or "stateless"
             Vec<String>, // State variables
             Vec<String>, // Hole variables
             Vec<String>), // Phv containers

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

pub enum Expr {
  // Captures parenthesis
  ExprWithParen (Box <Expr>),
  Num (i32),
  Var (String),
  Op (Box <Expr>, Opcode, Box<Expr> ),
  Mux3 (Box <Expr>, Box <Expr>, Box <Expr> ),
  Mux2 (Box <Expr>, Box <Expr>),
  Opt (Box <Expr>),
  ArithOp (Box<Expr>, Box<Expr>),
  Relop (Box<Expr>, Box<Expr>),
  Constant,
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

