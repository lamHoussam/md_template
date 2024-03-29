use crate::md_lexer::{MdToken, MdTokenType};
use std::{collections::{HashMap, VecDeque}, vec};

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
    End(MdTokenType),
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

    fn parse_string(&mut self, final_string: &mut String) {
        loop {
            let tk = self.tokens.pop_front().expect("Incomplete assignment");
            match tk.token_type { 
                MdTokenType::Operator('.') => {
                    let nxt = self.tokens.pop_front().expect("Need right part for string concat");
                    match nxt.token_type {
                        MdTokenType::String(val) => final_string.push_str(&val),
                        _ => println!("Cant concatenate string with other"),
                    }
                },
                MdTokenType::EndStatement => {
                    return;
                },
                // Handle Error
                _ => break,
            } 
        }
        
    }
    
    fn parse_assignment(&mut self, identifier: String) -> Statement {
        let op = self.tokens.pop_front().expect("Need operator here");
        // Handle Syntax Error
        if op.token_type != MdTokenType::Assign {
            return Statement::None;
        }
        
        // TODO: Implement different data assign
        let mut left: String = String::new();
        let frst = self.tokens.pop_front().expect("Need token here");
        match frst.token_type {
            MdTokenType::String(val) => {
                left.push_str(&val);
            },
            _ => println!("Need string here"),
        }

        self.parse_string(&mut left);
        return Statement::Assignment(Expr::Identifier(identifier), Expr::Litteral(Symbol::from(left)));
    }
    
    fn parse_if_statement(&mut self) -> Statement {
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
                    let sttment: Statement = self.parse_statement();
                    if sttment == Statement::End(MdTokenType::Else) {
                        break;
                    }
                    else if sttment == Statement::End(MdTokenType::Endif) {
                        return Statement::IfStatement(statements);
                    }
                    statements.push(sttment);
                }
                let mut if_count = 0;
                loop {
                    let tk = self.tokens.pop_front().expect("msg");
                    if tk.token_type == MdTokenType::If { if_count += 1; }
                    // println!("Token: {:?}", tk);
                    if tk.token_type == MdTokenType::Endif {
                        if if_count == 0 { break; }
                        if_count -= 1;
                    }
                }
                println!("Got if statement");
                return Statement::IfStatement(statements);
            },
            MdTokenType::False => {
                loop {
                    let tk = self.tokens.pop_front().expect("msg");
                    if tk.token_type == MdTokenType::Else || tk.token_type == MdTokenType::Endif {
                        break;
                    }
                }
                return Statement::IfStatement(vec![]);
            },
            _ => {
                println!("Handle error");
                return Statement::None;
            }
        }                    
        
        return Statement::None;
    }
    
    fn parse_for_statement(&mut self) -> Statement {
        return Statement::None;
    }

    fn parse_print_statement(&mut self) -> Statement {
        let mut printed_value: String = String::new();
        let tk = self.tokens.pop_front().expect("msg");
        match tk.token_type {
            MdTokenType::Dereference => {

            }, 
            MdTokenType::String(value)=> {
                printed_value.push_str(&value);
                self.parse_string(&mut printed_value);

                return Statement::PrintStatement(Expr::Litteral(Symbol::StringValue(printed_value)))
            },
            _ => {

            }
        }

        // println!("Print token: {:?}", tk.token_type);
        return Statement::PrintStatement(Expr::Identifier(tk.lexem));
    }
    
    fn parse_statement(&mut self) -> Statement {
        if let Some(token) = self.tokens.pop_front() {
            match token.token_type {
                MdTokenType::EndOfFile => { return Statement::End(token.token_type) },
                MdTokenType::If => {
                    return self.parse_if_statement();
                },
                MdTokenType::Print => {
                    return self.parse_print_statement();
                },
                MdTokenType::For => {
                    // TODO: Parse for statement
                    return Statement::None;
                }
                MdTokenType::Endfor | MdTokenType::Endif | MdTokenType::Else => {
                    return Statement::End(token.token_type);
                },
                MdTokenType::Identifier(identifier) => {
                    return self.parse_assignment(identifier);
                },
                _ => {
                    return Statement::None;
                },
            }
        }
        else { return Statement::End(MdTokenType::EndOfFile); }
    }

    fn execute_statement(&mut self, statement: Statement) {
        match statement {
            Statement::None => (),
            Statement::Assignment(identifier, value) => {
                if let Expr::Identifier(var_name) = identifier {
                    if let Expr::Litteral(symbol_value) = value {
                        self.global_variables.insert(var_name, symbol_value);
                    }
                }
            },
            Statement::IfStatement(in_statements) => {
                for sttmnt in in_statements {
                    self.execute_statement(sttmnt);
                }
            },
            Statement::ForLoop(statements) => {
                
            },
            Statement::PrintStatement(printed_value) => {
                
            },
            Statement::End(_) => (),
            Statement::Expr(_) => todo!(),
        }
    }

    pub fn parse(&mut self) {
        loop {
            let statement = self.parse_statement();
            println!("Statement: {:?}", statement);
            if statement == Statement::End(MdTokenType::EndOfFile) {
                break;
            }
            self.execute_statement(statement);
        }

        println!("Parser variables: {:?}", self.global_variables);
    }
}