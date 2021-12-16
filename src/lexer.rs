pub(crate) mod lexer {
    use std::collections::HashMap;
    use crate::lexer::lexer::Operations::{Add, And, Div, Mod, Mul, Not, Or, Sub};
    use crate::lexer::lexer::Constant::{False, Number, True};
    use crate::lexer::lexer::Token::{Colon, Comma, End, Op};
    use crate::Token::{Const, Lambda, LParen, RParen, Variable};

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub enum Operations {
        Mod,
        Add,
        Sub,
        Mul,
        Div,

        And,
        Or,
        Eq,
        Not,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Constant {
        True,
        False,
        Number(String),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Token {
        End,
        Lambda,
        Colon,
        Comma,
        LParen,
        RParen,
        Variable(String),
        Op(Operations),
        Const(Constant)
    }

    pub struct Lexer {
        cur_char: u8,
        cur_pos: usize,
        cur_token: Token,
        str: String,
    }

    impl Lexer {
        pub fn get(str: String) -> Lexer {
            let mut x = Lexer { cur_char: 0, cur_pos: 0, cur_token: Token::End, str };
            x.next_char();
            x.next_token();
            return x;
        }

        fn is_blank(c: u8) -> bool {
            return c == ' ' as u8 || c == '\r' as u8 || c == '\n' as u8;
        }

        fn is_digit(c: u8) -> bool {
            return c >= '0' as u8 && c <= '9' as u8;
        }

        fn is_separator(c: u8) -> bool {
            let seps = ['\0',
                '*', '/', '+', '-', '%',
                ',', ':',
                '=',
                '(', ')'];
            return Lexer::is_blank(c) || seps.contains(&(c as char));
        }

        fn is_letter(c: u8) -> bool {
            return (c >= 'a' as u8 && c <= 'z' as u8) || (c >= 'A' as u8 && c <= 'Z' as u8);
        }

        pub fn has_more_symbols(&self) -> bool {
            return self.cur_pos < self.str.len();
        }

        fn next_char(&mut self) {
            if !self.has_more_symbols() {
                self.cur_char = 0;
                return;
            }
            self.cur_char = self.str.as_bytes()[self.cur_pos];
            self.cur_pos = self.cur_pos + 1;
        }

        fn expect_str(&mut self, x: &str) {
            let mut i = 0;

            let s = x.as_bytes();
            while i < s.len() {
                self.next_char();
                assert_eq!(s[i], self.cur_char);
                i = i + 1;
            }
        }

        pub fn next_token(&mut self) {
            while Lexer::is_blank(self.cur_char) {
                self.next_char();
            }

            let cur_char = self.cur_char;

            if Lexer::is_separator(cur_char) {
                self.cur_token = match cur_char as char {
                    '\0' => End,
                    ',' => Comma,
                    ':' => Colon,
                    '(' => LParen,
                    ')' => RParen,
                    '=' => {
                        self.expect_str("=");
                        Op(Operations::Eq)
                    }
                    '*' => Op(Mul),
                    '/' => {
                        self.expect_str("/");
                        Op(Div)
                    }
                    '+' => Op(Add),
                    '-' => Op(Sub),
                    '%' => Op(Mod),
                    _ => panic!("Unknown separator")
                };
                self.next_char();
                return;
            }

            let mut is_literal = false;
            if Lexer::is_letter(self.cur_char) {
                is_literal = true;
            } else if Lexer::is_digit(self.cur_char) {
                is_literal = false;
            } else {
                panic!("Unexpected character '{}' at pos {}", self.cur_char as char, self.cur_pos);
            }
            let mut cur_tok = "".to_string();

            while !Lexer::is_separator(self.cur_char) {
                cur_tok = cur_tok + (self.cur_char as char).to_string().as_str();
                self.next_char();
            }

            if is_literal {
                let keywords = HashMap::from([
                    ("lambda", Lambda),
                    ("not", Op(Not)),
                    ("and", Op(And)),
                    ("or", Op(Or)),
                    ("True", Const(True)),
                    ("False", Const(False))
                ]);
                self.cur_token = match keywords.get(cur_tok.as_str()) {
                    None => Variable(cur_tok),
                    Some(tok) => tok.clone()
                };
            } else {
                self.cur_token = Const(Number(cur_tok));
            }
        }

        pub fn get_token(&self) -> Token {
            return self.cur_token.clone();
        }
    }
}
