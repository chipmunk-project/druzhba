pub mod ast;
// Important: nightly must be enabled to work
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub alugrammar); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_grammar;
