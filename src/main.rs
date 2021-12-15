mod lexer;

use crate::lexer::lexer::{Lexer, Token};
use crate::Token::End;

fn main() {
    let mut a = Lexer::get("lambda: (Hello == 42 || !(Hello == 24)), -92 - 152 * 3 / 2 + 33 -varvarvar * 92 %world".to_string());
    let mut tok: &Token;
    loop {
        a.next_token();
        tok = a.get_token();
        println!("{:?}", *tok);

        if *tok == End {
            break;
        }
    }
    println!("Hello, world!");
}
