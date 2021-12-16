# Python lambda expression parser

## Grammar
[Python language reference for expressions](https://docs.python.org/3/reference/expressions.html)

```
S -> lambda V : S'

# V -- parameter list
V  -> \String V' | ε
V' -> ,V | ε

# S is strictly lambda-expression
# S prime is either that or usual expr E
S' -> S
S' -> E

# E -- expression
E  -> O E'
E' -> or E | ε

# O -- `or` clause
O  -> A O'
O' -> and O | ε

# A -- `and` clause
A  -> not N | N

# N -- `not` clause
N  -> Q N'
N' -> == N | ε

# Q -- equality clause
Q  -> T Q'
Q' -> + Q | - Q | ε

# T -- term
T  -> F T'
T' -> * T | // T | % T | ε

# F -- factor
F  -> + C | - C | C

# C -- constant expression or variable or number
C  -> \String | \Num | True | False | (S')
```
Such grammar is LL(1), hence we can build an LL-parser for it.

## Lexical analyzer
Our grammar has the following non-terminals: `lambda`, `,`, `:`, `+`, `-`, `*`, `//`, `%`, `==`, `not`, `and`, `or`,
`(`, `)`, `True`, `False`, variable and integer literals. Let's also add a dummy token for EOL.
```rust
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

pub enum Constant {
    True,
    False,
    Number(String),
}

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
```

[Full source code for lexer](src/lexer.rs)

## Syntax analyzer
Let's first construct `FIRST` and `FOLLOW` sets for our grammar:

| Non-terminal | FIRST                                            | FOLLOW                              |
|--------------|--------------------------------------------------|-------------------------------------|
| S            | lambda                                           | ), EOL                              |
| V            | \String, ε                                       | :                                   |
| V'           | `,`, ε                                           | :                                   |
| S'           | lambda, \String, not, +, -, \Num, True, False, ( | ), EOL                              |
| E            | \String, not, +, -, \Num, True, False, (         | ), EOL                              |
| E'           | or, ε                                            | ), EOL                              |
| O            | \String, not, +, -, \Num, True, False, (         | or, ), EOL                          |
| O'           | and, ε                                           | or, ), EOL                          |
| A            | \String, not, +, -, \Num, True, False, (         | or, and, ), EOL                     |
| N            | \String, +, -, \Num, True, False, (              | or, and, ), EOL                     |
| N'           | ==, ε                                            | or, and, ), EOL                     |
| Q            | \String, +, -, \Num, True, False, (              | or, and, ==, ), EOL                 |
| Q'           | +, -, ε                                          | or, and, ==, ), EOL                 |
| T            | \String, +, -, \Num, True, False, (              | or, and, ==, +, -, ), EOL           |
| T'           | *, //, %, ε                                      | or, and, ==, +, -, ), EOL           |
| F            | \String, +, -, \Num, True, False, (              | or, and, ==, +, -, *, //, %, ), EOL |
| C            | \String, \Num, True, False, (                    | or, and, ==, +, -, *, //, %, ), EOL |

[Full source code for parser](src/parser.rs)

## Visualization
it literally prints GraphViz plot lmao.

## Tests
TBD.