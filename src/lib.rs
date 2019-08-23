pub mod pipeline_stage;
pub mod pipeline;
pub mod phv;
pub mod phv_container;
pub mod input_mux;
pub mod output_mux;
pub mod ast;
pub mod alu;

// Important: nightly must be enabled to work
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub alugrammar); // synthesized by LALRPOP

// Runs tests in test_druzhba.rs and
// test_grammar.rs
#[cfg(test)]

mod test_grammar;
mod test_druzhba;


