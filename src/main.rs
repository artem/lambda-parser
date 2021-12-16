mod lexer;
mod parser;

use petgraph::prelude::Graph;
use petgraph::dot::Dot;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::parser::Parser;
use crate::Token::End;

fn main() {
    let mut a = Parser::get("lambda: (Hello == 42 or not (Hello == 24)) -92 - 152 * 3 // 2 + 33 -varvarvar * 92 %world".to_string());
    a.S();
    println!("{}", Dot::new(&a.graph));
}
