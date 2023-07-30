pub mod symbol;

use pest::iterators::{Pairs, Pair};
use pest_derive::Parser;
use pest::Parser;
use std::collections::HashMap;
use std::fs::{self, File};

pub use symbol::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MdParser;


fn parse_string_expression(node: Pair<'_, Rule>, global_variables: &mut HashMap<String, Symbol>, local_variables: &mut HashMap<String, Symbol>) -> String {
    let mut final_string: String = String::new();
    match node.as_rule() {
        Rule::variable_interior => {
            let null_string: &Symbol = &Symbol::String(String::from("null"));
            let key: String = String::from(node.as_str());
            let var_value: &Symbol;
            
            match local_variables.get(&key) {
                Some(value) => var_value = value,
                None => {
                    match global_variables.get(&key) {
                        Some(value) => var_value = value,
                        None => var_value = &null_string,
                    }
                }
            }
            final_string.push_str(var_value.to_string().as_str());            

        }
        Rule::string => {
            let added_str: &str= node.as_str().trim_matches('\'');
            final_string.push_str(added_str);
        }
        _default => {
            let iter: Pairs<'_, Rule> = node.into_inner();
            for pair in iter {
                final_string.push_str(parse_string_expression(pair, global_variables, local_variables).as_str());
            }
        },
    }

    return final_string;
}

fn parse_syntax_tree(node: Pairs<'_, Rule, >, global_variables: &mut HashMap<String, Symbol>
                    , local_variables: &mut HashMap<String, Symbol>) {
    let iter: Pairs<'_, Rule> = node;
    for pair in iter {

        match pair.as_rule() {
            Rule::assignment_expression => {
                let assignment_exp: &str = pair.as_str();
                let vars: Vec<&str> = assignment_exp.split(":=").collect();
                if vars.len() != 2 {
                    println!("Error in variable declaration");
                    return;
                }

                let var_name: String = String::from(vars[0].trim());
                let var_value: String = String::from(vars[1].trim());

                println!("Variable: {:?}, Value: {:?}", var_name, var_value);
                
                let symb: Symbol = get_symbol_from_variable_value(var_value);
                
                // match symb {
                //     Symbol::String(_) => println!("IsString"),
                //     Symbol::Integer(_) => println!("IsInteger"),
                //     Symbol::Boolean(_) => println!("IsBoolean"),
                //     Symbol::Struct(_) => println!("IsStruct"),
                //     Symbol::List(_) => println!("IsList"),
                // }

                global_variables.insert(var_name, symb);
            }
            Rule::print_statement => {
                // println!("Print Expression: {:#?}", pair.clone().tokens());
                let node: Pairs<'_, Rule> = pair.clone().into_inner();
                match node.peek() {
                    Some(expression) => {
                        let printed_expression: String = parse_string_expression(expression, global_variables, local_variables);
                        println!("Expression: {}", printed_expression);
                    },
                    None => println!("Exmpty"),
                }
            }
            _default => {
                // println!("Other");
            }
        }

        let v = pair.into_inner();
        parse_syntax_tree(v, global_variables, local_variables);
    }
}

fn main() {
    let data_file_path: &str = "data/data.txt";
    let sample_file_path: &str = "test/sample.md";
    let output_file_path: &str = "output/file.md";

    let data_file: String = fs::read_to_string(data_file_path).expect("Failed to read data file");
    let sample_file: String = fs::read_to_string(sample_file_path).expect("Failed to read sample file.");
    
    let mut output_file: File = match File::create(output_file_path) {
        Ok(file) => file,
        Err(_) => panic!("Couldn't create output file"),
    };


    let mut global_variables: HashMap<String, Symbol> = HashMap::new();
    let mut local_variables: HashMap<String, Symbol> = HashMap::new();

    println!("Data file: ");
    match MdParser::parse(Rule::start, &data_file) {
        Ok(parsed) => {
            parse_syntax_tree(parsed, &mut global_variables, &mut local_variables);
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }

    println!("Variables: {:#?}", global_variables);

    println!("Sample file: ");
    match MdParser::parse(Rule::start, &sample_file) {
        Ok(parsed) => {
            parse_syntax_tree(parsed, &mut global_variables, &mut local_variables);

        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }
}