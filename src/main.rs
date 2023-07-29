use pest::iterators::Pairs;
use pest_derive::Parser;
use pest::{Parser, Token};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs, default};


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MdParser;

enum Symbol {
    String(String),
    Integer(i32),
    Boolean(bool),
    Struct(HashMap<String, Symbol>),
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").field(arg0).finish(),
        }
    }
}


fn parse_syntax_tree(node: Pairs<'_, Rule, >, variables: &mut HashMap<String, Symbol>) {
    let iter = node;
    for pair in iter {

        match pair.as_rule() {
            Rule::assignment_expression => {
                // let tokens = pair.clone().tokens();
                let assignment_exp: &str = pair.as_str();
                let vars: Vec<&str> = assignment_exp.split(":=").collect();
                if vars.len() != 2 {
                    println!("Error in variable declaration");
                    return;
                }

                let var_name: String = String::from(vars[0].trim());
                let var_value: String = String::from(vars[1].trim());

                println!("Variable: {:?}, Value: {:?}", var_name, var_value);
                variables.insert(var_name, Symbol::String(var_value));
            }
            _default => {
                println!("Other");
            }
        }

        let v = pair.into_inner();
        parse_syntax_tree(v, variables);
    }
}

fn main() {
    let data_file_path: &str = "data/data.txt";
    let sample_file_path: &str = "test/sample.md";

    let data_file: String = fs::read_to_string(data_file_path).expect("Failed to read data file");
    let sample_file: String = fs::read_to_string(sample_file_path).expect("Failed to read sample file.");
    
    let mut variables: HashMap<String, Symbol> = HashMap::new();

    println!("Data file: ");
    match MdParser::parse(Rule::start, &data_file) {
        Ok(parsed) => {
            parse_syntax_tree(parsed, &mut variables);
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }

    // println!("Variables: {:#?}", variables);
    for ele in variables {
        let k = ele.0;
        let v = ele.1;

        println!("Key: {:?}, Value: {:?}", k, v);
    }

    println!("Sample file: ");
    match MdParser::parse(Rule::start, &sample_file) {
        Ok(parsed) => {
            // for pair in parsed {
            //     println!("Rule: {:#?}, Span: {:#?}", pair.as_rule(), pair.as_span());
            // }
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }
}