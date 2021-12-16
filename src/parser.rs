pub(crate) mod parser {
    use crate::{Lexer, Token};
    use crate::lexer::lexer::Constant::{Number, True, False};
    use crate::lexer::lexer::Operations::{Add, And, Div, Eq, Mod, Mul, Not, Or, Sub};

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Tree {
        node: String,
        // TODO: unneeded?
        children: Vec<Tree>,
        id: i32,
    }

    impl Tree {
        pub fn get_leaf(id: i32, tok: String) -> Tree {
            return Tree { node: tok, children: vec![], id };
        }
    }

    pub struct Parser {
        pub(crate) lex: Lexer,
        id: i32, // TODO: unneeded?
    }

    impl Parser {
        pub fn get(str: String) -> Parser {
            return Parser{ lex: Lexer::get(str), id: 0 }
        }

        fn gid(&mut self) -> i32 {
            let i = self.id;
            self.id = i + 1;
            return i;
        }

        pub fn S(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "S".to_string());

            match self.lex.get_token() {
                Token::Lambda => {
                    // lambda
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "lambda".to_string()));
                    // V
                    node.children.push(self.V());
                    // :
                    assert_eq!(self.lex.get_token(), Token::Colon);
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), ":".to_string()));
                    // S'
                    node.children.push(self.Sp());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn V(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "V".to_string());

            match self.lex.get_token() {
                Token::Variable(str) => {
                    // var
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), str));
                    // V'
                    node.children.push(self.Vp());
                }
                Token::Colon => {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Vp(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "V'".to_string());

            match self.lex.get_token() {
                Token::Comma => {
                    // var
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), ",".to_string()));
                    // V'
                    node.children.push(self.V());
                }
                Token::Colon => {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Sp(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "S'".to_string());

            match self.lex.get_token() {
                Token::Lambda => {
                    // S
                    node.children.push(self.S());
                }
                Token::Variable(_)
                | Token::Op(Not)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => { node.children.push(self.E()); }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn E(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "E".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Not)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // O
                    node.children.push(self.O());
                    // E'
                    node.children.push(self.Ep());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Ep(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "Ep".to_string());

            match self.lex.get_token() {
                Token::Op(Or) => {
                    // or
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "or".to_string()));
                    // E
                    node.children.push(self.E());
                }
                Token::RParen | Token::End => {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn O(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "O".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Not)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // A
                    node.children.push(self.A());
                    // O'
                    node.children.push(self.Op());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Op(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "Op".to_string());

            match self.lex.get_token() {
                Token::Op(And) => {
                    // and
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "and".to_string()));
                    // O
                    node.children.push(self.O());
                }
                Token::RParen
                | Token::End
                | Token::Op(Or) => {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn A(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "A".to_string());

            match self.lex.get_token() {
                Token::Op(Not) => {
                    // not
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "not".to_string()));
                    // N
                    node.children.push(self.N());
                }
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // N
                    node.children.push(self.N());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn N(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "N".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // Q
                    node.children.push(self.Q());
                    // N'
                    node.children.push(self.Np());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Np(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "Np".to_string());

            match self.lex.get_token() {
                Token::Op(Eq) => {
                    // ==
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "==".to_string()));
                    // N
                    node.children.push(self.N());
                }
                Token::RParen
                | Token::End
                | Token::Op(Or)
                | Token::Op(And) => {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Q(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "Q".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // T
                    node.children.push(self.T());
                    // Q'
                    node.children.push(self.Qp());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Qp(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "Qp".to_string());

            match self.lex.get_token() {
                Token::Op(Add) => {
                    // +
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "+".to_string()));
                    // Q
                    node.children.push(self.Q());
                }
                Token::Op(Sub) => {
                    // -
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "-".to_string()));
                    // Q
                    node.children.push(self.Q());
                }
                Token::RParen
                | Token::End
                | Token::Op(Or)
                | Token::Op(And)
                | Token::Op(Eq)=> {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn T(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "T".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // F
                    node.children.push(self.F());
                    // T'
                    node.children.push(self.Tp());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Tp(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "Tp".to_string());

            match self.lex.get_token() {
                Token::Op(Mul) => {
                    // *
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "*".to_string()));
                    // T
                    node.children.push(self.T());
                }
                Token::Op(Div) => {
                    // //
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "//".to_string()));
                    // T
                    node.children.push(self.T());
                }
                Token::Op(Mod) => {
                    // %
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "%".to_string()));
                    // T
                    node.children.push(self.T());
                }
                Token::RParen
                | Token::End
                | Token::Op(Or)
                | Token::Op(And)
                | Token::Op(Eq)
                | Token::Op(Add)
                | Token::Op(Sub)=> {}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn F(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "F".to_string());

            match self.lex.get_token() {
                Token::Op(Add) => {
                    // +
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "+".to_string()));
                    // C
                    node.children.push(self.C());
                }
                Token::Op(Sub) => {
                    // -
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "-".to_string()));
                    // C
                    node.children.push(self.C());
                }
                Token::Variable(_)
                | Token::Const(_)
                | Token::LParen => {
                    // C
                    node.children.push(self.C());
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn C(&mut self) -> Tree {
            let mut node = Tree::get_leaf(self.gid(), "C".to_string());

            match self.lex.get_token() {
                Token::Variable(str)
                | Token::Const(Number(str)) => {
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), str));
                }
                Token::Const(True) => {
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "True".to_string()));
                }
                Token::Const(False) => {
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "False".to_string()));
                }
                Token::LParen => {
                    // (
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), "(".to_string()));
                    // S'
                    node.children.push(self.Sp());
                    // )
                    assert_eq!(self.lex.get_token(), Token::RParen);
                    self.lex.next_token();
                    node.children.push(Tree::get_leaf(self.gid(), ")".to_string()));
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }
    }
}
