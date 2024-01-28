use crate::md_lexer::{MdToken, MdTokenType};
use std::collections::{HashMap, VecDeque};

// Define Operationals?

#[derive(Debug, PartialEq)]
enum Expr {
    Litteral(Symbol),
    Identifier(String),
    Operation(Box<Expr>, char, Box<Expr>),
    None,
}

#[derive(Debug, PartialEq)]
enum Statement {
    Assignment(Expr, Expr),
    IfStatement(Vec<Statement>),
    ForLoop(Vec<Statement>),
    PrintStatement(Expr),
    Expr(Expr),
    None,
    End,
}

#[derive(Debug, PartialEq)]
enum Symbol {
    StringValue(String),
    IntegerValue(i32),
    FloatValue(f32),
    ArrayValue(Vec<Symbol>),
}

impl Symbol {
    pub fn from(litteral: String) -> Self {
        Symbol::StringValue(litteral)
    }
}

pub struct MdParser<'a> {
    tokens: &'a mut VecDeque<MdToken>,
    statements: Vec<Statement>,
    local_variables: HashMap<String, Symbol>,
    global_variables: HashMap<String, Symbol>,
}


impl<'a> MdParser<'a> {
    pub fn new(token_stream: &'a mut VecDeque<MdToken>) -> Self {
        MdParser {
            tokens: token_stream,
            statements: Vec::new(),
            local_variables: HashMap::new(),
            global_variables: HashMap::new(),
        }
    }

    fn parse_expression(&mut self) -> Expr {
        let token = self.tokens.pop_front().expect("msg");
        let t_type = token.token_type;
        match t_type {
            MdTokenType::Identifier(identifier) => Expr::Identifier(identifier),
            MdTokenType::Number(number) => Expr::Litteral(Symbol::IntegerValue(number)),
            MdTokenType::String(value) => Expr::Litteral(Symbol::StringValue(value)),
            _ => Expr::None,
        }
    }

    // fn parse_if_statement(&mut self) {
    //     let condition = self.tokens.pop_front().expect("If needs condition");
    //     if condition.token_type == MdTokenType::Dereference {
    //         let expr = self.parse_expression();
    //         println!("Expr: {:?}", expr);
    //     }

        
    // }

    fn parse_assignment(&mut self) {

    }

    fn parse_statement(&mut self) -> Statement {
        if let Some(token) = self.tokens.pop_front() {
            match token.token_type {
                MdTokenType::If => {
                    let mut condition: Vec<MdToken> = Vec::new();
                    loop {
                        let tk = self.tokens.pop_front().expect("Error");
                        if tk.token_type == MdTokenType::Then {
                            break;
                        }
                        condition.push(tk);
                    }
                    // For now we only take one token as condition, later need to implement Operations
                    let cond = condition.first().expect("Error");
                    let mut statements: Vec<Statement> = Vec::new();
                    match cond.token_type {
                        MdTokenType::Dereference => {
                            // TODO: Evaluate dereferenced value
                        },
                        MdTokenType::True => {
                            loop {
                                let sttment = self.parse_statement();
                                if sttment == Statement::None {
                                    break;
                                }
                                statements.push(sttment);
                            }

                            return Statement::IfStatement(statements);
                        },
                        MdTokenType::False => {
                            loop {
                                let tk = self.tokens.pop_front().expect("msg");
                                if tk.token_type == MdTokenType::Else || tk.token_type == MdTokenType::Endif {
                                    break;
                                }
                            }
                            return Statement::None;
                        },
                        _ => {
                            println!("Handle error");
                            return Statement::None;
                        }
                    }                    

                    return Statement::None;
                },
                MdTokenType::Endfor => {
                    return Statement::End;
                },
                MdTokenType::Identifier(identifier) => {
                    let op = self.tokens.pop_front().expect("Need operator here");
                    // Handle Syntax Error
                    if op.token_type != MdTokenType::Assign {
                        return Statement::None;
                    }

                    loop {
                        let tk = self.tokens.pop_front().expect("Incomplete assignment");
                        match tk.token_type {
                            MdTokenType::String(litteral) => {
                                return Statement::Assignment(Expr::Identifier(identifier), Expr::Litteral(Symbol::from(litteral)));
                            }
                            // Handle Error
                            _ => break,
                        }                        
                    }

                    return Statement::None;
                },
                MdTokenType::EndOfFile => Statement::None,
                _ => Statement::None,
            };
        }

        return Statement::None;
    }

    pub fn parse(&mut self) {
        loop {
            let statement = self.parse_statement();
            println!("{:#?}", statement);
            if statement == Statement::End {
                break;
            }
        }
    }
}