use std::{str::Chars, iter::Peekable, collections::{HashMap, LinkedList, self, VecDeque}, fmt::Debug};


#[derive(Debug, PartialEq, Clone)]
pub enum MdTokenType {
    MdText,
    Identifier(String),
    String(String),
    Number(i32),
    Operator(char),
    Dereference,
    
    If,
    Then,
    Else,
    Endif,
    For,
    In,
    Do,
    Endfor,
    EndOfFile,
    Print,
    True,
    False,
    
    // 2 char Tokens
    Assign,
    CodeStart,
    CodeEnd,
    EndStatement,

    Unknown(char),
}

#[derive(Debug)]
pub struct MdToken {
    pub token_type: MdTokenType,
    pub lexem: String,
    pub line: i64,
}

#[derive(Debug)]
enum LexerError {
    UnknownChar(MdToken)
}

pub struct MdLexer<'a> {
    source_code: Peekable<Chars<'a>>,
    pub tokens: VecDeque<MdToken>,
    current_pos: i64,
    keywords: HashMap<&'a str, MdTokenType>,
    is_code_bloc: bool,
}

// keywords: Map<String, TokenType>;

impl<'a> MdLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut kw = HashMap::new();

        kw.insert("if", MdTokenType::If);
        kw.insert("then", MdTokenType::Then);
        kw.insert("else", MdTokenType::Else);
        kw.insert("endif", MdTokenType::Endif);
        kw.insert("for", MdTokenType::For);
        kw.insert("in", MdTokenType::In);
        kw.insert("do", MdTokenType::Do);
        kw.insert("endfor", MdTokenType::Endfor);
        kw.insert("print", MdTokenType::Print);
        kw.insert("True", MdTokenType::True);
        kw.insert("False", MdTokenType::False);

        MdLexer {
            source_code: input.chars().peekable(),
            tokens: VecDeque::new(),
            current_pos: 1,
            keywords: kw,
            is_code_bloc: false,
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&ch) = self.source_code.peek() {
            if ch == '\n' {
                self.current_pos += 1;
            }
            else if !ch.is_whitespace() {
                break;
            }
            self.source_code.next();
        }
    }

    fn scan_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(&ch) = self.source_code.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.source_code.next();
            } else {
                break;
            }
        }

        identifier
    }

    fn scan_number_unparsed(&mut self) -> String {
        let mut number = String::new();

        while let Some(&ch) = self.source_code.peek() {
            if ch.is_digit(10) {
                number.push(ch);
                self.source_code.next();
            } else {
                break;
            }
        }

        number
    }

    // TODO: Add Error Handling
    fn scan_string(&mut self) -> String {
        let mut res: String = String::new();
        self.source_code.next();
        while let Some(&ch) = self.source_code.peek() {
            if ch == '"' { 
                self.source_code.next();
                return res; 
            }
            res.push(ch);
            self.source_code.next();
        }

        res
    }

    fn scan_md_txt(&mut self) -> String {
        let mut txt = String::new();
        while let Some(&ch) = self.source_code.peek() {
            match ch {
                '{' => {
                    self.source_code.next();
                    if let Some(&ch2) = self.source_code.peek() {
                        if ch2 == '{' {
                            self.source_code.next();
                            self.is_code_bloc = true;
                            return txt;
                        }
                    }
                    
                    txt.push(ch);
                }
                '\n' => {
                    self.source_code.next();
                    txt.push(ch);
                    self.current_pos += 1;
                }
                _ => {
                    self.source_code.next();
                    txt.push(ch);
                }
            }
        }

        self.is_code_bloc = true;
        txt
    }

    fn next_token(&mut self) -> Result<MdToken, LexerError> {
        // TODO: Refactor
        if !self.is_code_bloc {
            let txt = self.scan_md_txt();
            return Ok(MdToken {token_type: MdTokenType::MdText, line: self.current_pos, lexem: txt });
        }

        self.consume_whitespace();

        if let Some(&ch) = self.source_code.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let value = self.scan_identifier();
                    match self.keywords.get(value.as_str()) {
                        Some(ttype) => {
                            return Ok(MdToken { token_type: ttype.clone(), line: self.current_pos, lexem: value });
                        },
                        None => {
                            return Ok(MdToken { token_type: MdTokenType::Identifier(value.clone()), line: self.current_pos, lexem: value });
                        },
                    }
                } 
                '"' => {
                    let value = self.scan_string();
                    return Ok(MdToken { token_type: MdTokenType::String(value.clone()), line: self.current_pos, lexem: value });
                }
                '0'..='9' => {
                    let value = self.scan_number_unparsed();
                    return Ok(MdToken { token_type: MdTokenType::Number(value.parse().unwrap_or(0)), line: self.current_pos, lexem: value });
                }
                '+' | '-' | '*' | '/' | '.' => {
                    self.source_code.next();
                    return Ok(MdToken { token_type: MdTokenType::Operator(ch), line: self.current_pos, lexem: ch.to_string() });
                }
                ':' => {
                    self.source_code.next();
                    if let Some(&ch2) = self.source_code.peek() {
                        if ch2 == '=' {
                            self.source_code.next();
                            return Ok(MdToken { token_type: MdTokenType::Assign, line: self.current_pos, lexem: ":=".to_string() });
                        }
                    }
                    return Ok(MdToken { token_type: MdTokenType::Unknown(ch), line: self.current_pos, lexem: ch.to_string() });
                }
                '$' => {
                    self.source_code.next();
                    let identifier = self.scan_identifier();
                    return Ok(MdToken { token_type: MdTokenType::Dereference, line: self.current_pos, lexem: identifier });
                }
                '}' => {
                    self.source_code.next();
                    if let Some(&ch2) = self.source_code.peek() {
                        if ch2 == '}' {
                            self.source_code.next();
                            self.is_code_bloc = false;
                            return Ok(MdToken { token_type: MdTokenType::CodeEnd, line: self.current_pos, lexem: "}}".to_string() });
                        }
                    }
                    return Err(LexerError::UnknownChar(MdToken { token_type: MdTokenType::Unknown(ch), line: self.current_pos, lexem: ch.to_string() }));
                }
                ';' => {
                    self.source_code.next();
                    if let Some(&ch2) = self.source_code.peek() {
                        if ch2 == ';' {
                            self.source_code.next();
                            return Ok(MdToken { token_type: MdTokenType::EndStatement, line: self.current_pos, lexem: ";;".to_string() });
                        }
                    }
                    return Err(LexerError::UnknownChar(MdToken { token_type: MdTokenType::Unknown(ch), line: self.current_pos, lexem: ch.to_string() }));
                }
                _ => {
                    self.source_code.next();
                    return Err(LexerError::UnknownChar(MdToken { token_type: MdTokenType::Unknown(ch), line: self.current_pos, lexem: ch.to_string() }));
                }
            }
        } else {
            return Ok(MdToken { token_type: MdTokenType::EndOfFile, line: self.current_pos, lexem: String::new() });
        }
    }


    pub fn scan_tokens(&mut self) -> Option<MdToken> {
        loop {
            let token = self.next_token();
            match token {
                Ok(tkn) => {
                    println!("{:?}", tkn);
                    if tkn.token_type == MdTokenType::EndOfFile {
                        return Some(tkn);
                    }
        
                    self.tokens.push_back(tkn);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    return None;
                },
            }
        }

    }
}
