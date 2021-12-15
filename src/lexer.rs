mod lexer {
    use std::path::is_separator;
    use crate::lexer::lexer::Arith::{Add, Div, Mod, Mul, Sub};
    use crate::lexer::lexer::Log::{And, Or};
    use crate::lexer::lexer::Token::{Colon, Comma, End, LogOp};

    enum Arith {
        Mod,
        Add,
        Sub,
        Mul,
        Div,
    }

    enum Log {
        And,
        Or,
        Eq,
    }

    enum Token {
        End,
        Lambda,
        Colon,
        Comma,
        Number,
        Variable(String),
        IntOp(Arith),
        LogOp(Log),
    }

    struct Lexer {
        cur_char: u8,
        cur_pos: usize,
        cur_token: Token,
        str: String,
    }

    impl Lexer {
        fn get(str: String) -> Lexer {
            let mut x = Lexer { cur_char: 0, cur_pos: 0, cur_token: Token::NONE, str };
            x.next_char();
            return x;
        }

        fn is_blank(c: u8) -> bool {
            return c == ' ' as u8 || c == '\r' as u8 || c == '\n' as u8;
        }

        fn is_digit(c: u8) -> bool {
            return c >= '0' as u8 && c <= '9' as u8;
        }

        fn is_separator(c: u8) -> bool {
            let seps = ['*', '/', '+', '-', '%', ',', ':'];
            return Lexer::is_blank(c) || seps.contains(&(c as char));
        }

        fn is_var_first(c: u8) -> bool {
            return !Lexer::is_separator(c) && !Lexer::is_digit(c);
        }

        fn next_char(&mut self) {
            assert!(self.cur_pos < self.str.len());
            self.cur_char = self.str.as_bytes()[self.cur_pos];
            self.cur_pos = self.cur_pos + 1;
        }

        fn next_token(&mut self) {
            while Lexer::is_blank(self.cur_char) {
                Lexer::next_char(self);
            }

            let cur_char = self.cur_char;

            if self.cur_pos >= self.str.len() {
                self.cur_token = End;
                return;
            }

            if Lexer::is_separator(cur_char) {
                self.cur_token = match cur_char as char {
                    ',' => Comma,
                    ':' => Colon,
                    '&' => {
                        expect_str("&");
                        LogOp(And)
                    }
                    '|' => {
                        expect_str("|");
                        LogOp(Or)
                    }
                    '*' => Arith(Mul),
                    '/' => Arith(Div),
                    '+' => Arith(Add),
                    '-' => Arith(Sub),
                    '%' => Arith(Mod),
                    _ => panic!("Unknown separator")
                };
                Lexer::next_char(self);
                return;
            }
        }
    }
}
