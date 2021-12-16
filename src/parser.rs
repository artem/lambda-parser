pub(crate) mod parser {
    use petgraph::Graph;
    use petgraph::graph::NodeIndex;

    use crate::{Lexer, Token};
    use crate::lexer::lexer::Constant::{False, Number, True};
    use crate::lexer::lexer::Operations::{Add, And, Div, Eq, Mod, Mul, Not, Or, Sub};

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Tree {
        
    }

    impl Tree {
        pub fn get_leaf(graph: &mut Graph<String, &str>, tok: String) -> NodeIndex {
            return graph.add_node(tok);
        }
    }

    pub struct Parser<'a> {
        pub(crate) lex: Lexer,
        pub(crate) graph: Graph<String, &'a str>,
    }

    impl Parser<'_> {
        pub fn get(str: String) -> Parser<'static> {
            return Parser { lex: Lexer::get(str), graph: Graph::new() };
        }
        
        fn add_eps_node(&mut self, p: NodeIndex) {
            let xxx = Tree::get_leaf(&mut self.graph, "ε".to_string());
            self.graph.add_edge(p, xxx, "");
        }

        pub fn S(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "S".to_string());

            match self.lex.get_token() {
                Token::Lambda => {
                    // lambda
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "lambda".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // V
                    let xxx = self.V();
                    self.graph.add_edge(node, xxx, "");
                    // :
                    assert_eq!(self.lex.get_token(), Token::Colon);
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, ":".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // S'
                    let xxx = self.Sp();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn V(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "V".to_string());

            match self.lex.get_token() {
                Token::Variable(str) => {
                    // var
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, str);
                    self.graph.add_edge(node, xxx, "");
                    // V'
                    let xxx = self.Vp();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Colon => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Vp(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "V'".to_string());

            match self.lex.get_token() {
                Token::Comma => {
                    // var
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, ",".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // V'
                    let xxx = self.V();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Colon => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Sp(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "S'".to_string());

            match self.lex.get_token() {
                Token::Lambda => {
                    // S
                    let xxx = self.S();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Variable(_)
                | Token::Op(Not)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    let xxx = self.E();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn E(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "E".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Not)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // O
                    let xxx = self.O();
                    self.graph.add_edge(node, xxx, "");
                    // E'
                    let xxx = self.Ep();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Ep(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "Ep".to_string());

            match self.lex.get_token() {
                Token::Op(Or) => {
                    // or
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "or".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // E
                    let xxx = self.E();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::RParen | Token::End => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn O(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "O".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Not)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // A
                    let xxx = self.A();
                    self.graph.add_edge(node, xxx, "");
                    // O'
                    let xxx = self.Op();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Op(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "Op".to_string());

            match self.lex.get_token() {
                Token::Op(And) => {
                    // and
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "and".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // O
                    let xxx = self.O();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::RParen
                | Token::End
                | Token::Op(Or) => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn A(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "A".to_string());

            match self.lex.get_token() {
                Token::Op(Not) => {
                    // not
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "not".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // N
                    let xxx = self.N();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // N
                    let xxx = self.N();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn N(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "N".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // Q
                    let xxx = self.Q();
                    self.graph.add_edge(node, xxx, "");
                    // N'
                    let xxx = self.Np();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Np(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "Np".to_string());

            match self.lex.get_token() {
                Token::Op(Eq) => {
                    // ==
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "==".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // N
                    let xxx = self.N();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::RParen
                | Token::End
                | Token::Op(Or)
                | Token::Op(And) => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Q(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "Q".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // T
                    let xxx = self.T();
                    self.graph.add_edge(node, xxx, "");
                    // Q'
                    let xxx = self.Qp();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Qp(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "Qp".to_string());

            match self.lex.get_token() {
                Token::Op(Add) => {
                    // +
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "+".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // Q
                    let xxx = self.Q();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Op(Sub) => {
                    // -
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "-".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // Q
                    let xxx = self.Q();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::RParen
                | Token::End
                | Token::Op(Or)
                | Token::Op(And)
                | Token::Op(Eq) => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn T(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "T".to_string());

            match self.lex.get_token() {
                Token::Variable(_)
                | Token::Op(Add)
                | Token::Op(Sub)
                | Token::Const(_)
                | Token::LParen => {
                    // F
                    let xxx = self.F();
                    self.graph.add_edge(node, xxx, "");
                    // T'
                    let xxx = self.Tp();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn Tp(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "Tp".to_string());

            match self.lex.get_token() {
                Token::Op(Mul) => {
                    // *
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "*".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // T
                    let xxx = self.T();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Op(Div) => {
                    // //
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "//".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // T
                    let xxx = self.T();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Op(Mod) => {
                    // %
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "%".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // T
                    let xxx = self.T();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::RParen
                | Token::End
                | Token::Op(Or)
                | Token::Op(And)
                | Token::Op(Eq)
                | Token::Op(Add)
                | Token::Op(Sub) => {self.add_eps_node(node)}
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn F(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "F".to_string());

            match self.lex.get_token() {
                Token::Op(Add) => {
                    // +
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "+".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // C
                    let xxx = self.C();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Op(Sub) => {
                    // -
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "-".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // C
                    let xxx = self.C();
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Variable(_)
                | Token::Const(_)
                | Token::LParen => {
                    // C
                    let xxx = self.C();
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }

        pub fn C(&mut self) -> NodeIndex {
            let node = Tree::get_leaf(&mut self.graph, "C".to_string());

            match self.lex.get_token() {
                Token::Variable(str)
                | Token::Const(Number(str)) => {
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, str);
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Const(True) => {
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "True".to_string());
                    self.graph.add_edge(node, xxx, "");
                }
                Token::Const(False) => {
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "False".to_string());
                    self.graph.add_edge(node, xxx, "");
                }
                Token::LParen => {
                    // (
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, "(".to_string());
                    self.graph.add_edge(node, xxx, "");
                    // S'
                    let xxx = self.Sp();
                    self.graph.add_edge(node, xxx, "");
                    // )
                    assert_eq!(self.lex.get_token(), Token::RParen);
                    self.lex.next_token();
                    let xxx = Tree::get_leaf(&mut self.graph, ")".to_string());
                    self.graph.add_edge(node, xxx, "");
                }
                tok => panic!("Unexpected token {:?}", tok)
            }

            return node;
        }
    }
}
