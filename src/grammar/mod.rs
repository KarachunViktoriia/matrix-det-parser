extern crate pest_derive;
extern crate pest;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MatrixParser;