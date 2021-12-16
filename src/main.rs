mod lexer;
mod parser;

use crate::lexer::lexer::{Lexer, Token};
use crate::parser::parser::Parser;
use crate::Token::End;

fn main() {
    // let mut a = Lexer::get("lambda: (Hello == 42 or not (Hello == 24)) -92 - 152 * 3 // 2 + 33 -varvarvar * 92 %world".to_string());
    // let mut tok: Token;
    // loop {
    //     a.next_token();
    //     tok = a.get_token();
    //     println!("{:?}", tok);
    //
    //     if tok == End {
    //         break;
    //     }
    // }
    let mut a = Parser::get("lambda: (Hello == 42 or not (Hello == 24)) -92 - 152 * 3 // 2 + 33 -varvarvar * 92 %world".to_string());
    println!("{:?}", a.S());
    println!("Hello, world!");
}
