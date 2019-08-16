use std::fs;
use ast;
use alugrammar;

#[test]
pub fn test_expr() {
  assert!(alugrammar::ExprParser::new().parse("((((C()))))").is_ok());
  assert!(alugrammar::ExprParser::new().parse("((Opt(C()))))").is_err());
  assert!(alugrammar::ExprParser::new().parse("2+2==3").is_ok());

  assert!(alugrammar::ExprParser::new().parse("thisIsAVar").is_ok());
  assert!(alugrammar::ExprParser::new().parse("2 +20>=4*2").is_ok());
  assert!(alugrammar::ExprParser::new().parse("Mux3 (arith_op (2,4), 12, Opt(10<2) )").is_ok());

}
#[test]
pub fn test_stmt() {
  assert!(alugrammar::StmtParser::new().parse("return Mux3 (Opt(5),12-5*2, 10);").is_ok());

  assert!(alugrammar::StmtParser::new().parse("return (pkt_0 != 0 || immediate_operand != 0);").is_ok());
  assert!(alugrammar::StmtParser::new().parse("if (2 == 2){ return 5; }").is_ok());
  assert! (alugrammar::StmtParser::new().parse(
          " if (Opt (3)) {
                return 1;
            } elif (2 == 2) {
                return 0;
            } else {
                return 4;
            }" ).is_ok());

  assert! (alugrammar::StmtParser::new().parse(
          " if (Opt (pkt_1)) {
                return 1;
            } elif (Opt(pkt_0)) {
                return 0;
            }elif (rel_op(pkt_0, pkt_1)){
                return 2;   
            }
            elif (Mux3(pkt_0, pkt_1, C()) < arith_op (state_0,pkt_1)){
                return 10;
            } else {
                return 2;   
            }").is_ok());

  assert! (alugrammar::StmtParser::new().parse("x = 3;").is_ok());

  assert! (alugrammar::StmtParser::new().parse("state_0 = 9;").is_ok());
  assert! (alugrammar::StmtParser::new().parse("state_0 = pkt_2 + Opt(pkt_1);").is_ok());

}
#[test]
pub fn test_header ()
{
  assert! (alugrammar::HeaderParser::new().parse(
          " type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}"
          ).is_ok());
  assert! (alugrammar::HeaderParser::new().parse(
          " type : state
            state variables : {state_0}
            hole variables : {opcode}
            packet fields : {pkt_0}"
          ).is_err()); 
  assert! (alugrammar::HeaderParser::new().parse(
          " type : stateful
            state variables : {state_0, state_1}
            hole variables : {}
            packet fields : {pkt_0}"
          ).is_ok());
  assert! (alugrammar::HeaderParser::new().parse(
          " type : stateful
            statee variables : {state_0}
            hole variables : {opcode}
            packet fields : {pkt_0}"
          ).is_err()); 
}
#[test]
pub fn test_alugrammar ()
{
  assert! (alugrammar::AluParser::new().parse(
          " type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}

            if (opcode==0) {
                return pkt_1;
            } elif (opcode==1) {
                return pkt_0;
            } elif (opcode==2){
                return pkt_1 +immediate_operand;   
            }
            elif (opcode==3){
                return pkt_0 + immediate_operand;
            } else {
                return pkt_1;   
            }
            ").is_ok());
}
#[test]
pub fn test_optheader ()
{
  assert! (alugrammar::AluParser::new().parse(
          " name : times_two_if_else_raw_3_3
            pipeline stage : 2
            alu : 0
            type : stateful
            state variables : {state_0}
            hole variables : {}
            packet fields : {pkt_0}

            state_0 = 0;
            
            ").is_ok());

  assert! (alugrammar::AluParser::new().parse(
          " name : times_two_if_else_raw_3_3
            pipeline stage : 2
            alu : 0
            type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}

            if (opcode==0) {
                return pkt_1;
            } elif (opcode==1) {
                return pkt_0;
            } elif (opcode==2){
                return pkt_1 +immediate_operand;   
            }
            elif (opcode==3){
                return pkt_0 + immediate_operand;
            } else {
                return pkt_1;   
            }
            ").is_ok());
}

#[test]
pub fn test_comment()
{

  assert! (alugrammar::CommentParser::new().parse(
          "// This is a comment").is_ok());
  assert! (alugrammar::CommentParser::new().parse(
          "// This is a number 15").is_ok());

}
#[test]
pub fn test_stateless_alus ()
{

  let alu1 = fs::read_to_string("example_alus/stateless_alus/stateless_alu_arith.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu1).is_ok());

  let alu2 = fs::read_to_string("example_alus/stateless_alus/stateless_alu.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu2).is_ok());

   let alu3 = fs::read_to_string("example_alus/stateless_alus/stateless_alu_arith.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu3).is_ok());

 let alu4 = fs::read_to_string("example_alus/stateless_alus/stateless_alu_arith.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu4).is_ok());

  let alu5 = fs::read_to_string("example_alus/stateless_alus/stateless_alu_arith.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu5).is_ok());

}
#[test]
pub fn test_stateful_alus ()
{
  let alu1 = fs::read_to_string("example_alus/stateful_alus/raw.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu1).is_ok());

  let alu2 = fs::read_to_string("example_alus/stateful_alus/sub.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu2).is_ok());

   let alu3 = fs::read_to_string("example_alus/stateful_alus/if_else_raw.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu3).is_ok());

 let alu4 = fs::read_to_string("example_alus/stateful_alus/nested_ifs.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu4).is_ok());

  let alu5 = fs::read_to_string("example_alus/stateful_alus/pred_raw.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu5).is_ok());

  let alu6 = fs::read_to_string("example_alus/stateful_alus/pair.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu6).is_ok());

}

