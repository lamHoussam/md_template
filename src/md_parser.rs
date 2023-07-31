use std::collections::HashMap;
use pest::iterators::{Pairs, Pair};
use crate::{Symbol, get_symbol_from_variable_value};



#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct MdParser;

impl MdParser {
    pub fn parse_string_expression(node: Pair<'_, Rule>, global_variables: &mut HashMap<String, Symbol>, local_variables: &mut HashMap<String, Symbol>) -> String {
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
                let mut added_str: String = String::from(node.as_str().to_string());
    
                added_str = added_str.replace(r#"\'"#, "\'")
                                    .replace(r#"\\"#, "\\")
                                    .replace(r#"\n"#, "\n")
                                    .replace(r#"\t"#, "\t")
                                    .replace(r#"\r"#, "\r")
                                    .replace(r#"\0"#, "\0");
    
                // println!("Node {:?}", added_str);
                added_str = added_str[1..(added_str.len() - 1)].to_string();
    
                final_string.push_str(&added_str);
            }
            _default => {
                let iter: Pairs<'_, Rule> = node.into_inner();
                for pair in iter {
                    final_string.push_str(MdParser::parse_string_expression(pair, global_variables, local_variables).as_str());
                }
            },
        }
    
        return final_string;
    }
    
    pub fn parse_syntax_tree(node: Pairs<'_, Rule, >, global_variables: &mut HashMap<String, Symbol>, 
    local_variables: &mut HashMap<String, Symbol>) -> Option<String> {
    
        let iter: Pairs<'_, Rule> = node;
        let mut output_string: String = String::new();
    
        for pair in iter {
    
            match pair.as_rule() {
                Rule::assignment_expression => {
                    let assignment_exp: &str = pair.as_str();
                    let vars: Vec<&str> = assignment_exp.split(":=").collect();
                    if vars.len() != 2 {
                        println!("Error in variable declaration");
                        return None;
                    }
    
                    let var_name: String = String::from(vars[0].trim());
                    let var_value: String = String::from(vars[1].trim());
    
                    println!("Variable: {:?}, Value: {:?}", var_name, var_value);
                    
                    let symb: Symbol = get_symbol_from_variable_value(var_value);
    
                    global_variables.insert(var_name, symb);
                }
                Rule::print_statement => {
                    // println!("Print Expression: {:#?}", pair.clone().tokens());
                    let node: Pairs<'_, Rule> = pair.clone().into_inner();
                    match node.peek() {
                        Some(expression) => {
                            let printed_expression: String = MdParser::parse_string_expression(expression, global_variables, local_variables);                        
                            
                            output_string.push_str(&printed_expression);
                            // println!("{}", printed_expression);
                        },
                        None => println!("Exmpty"),
                    }
                }
                Rule::txt => {
                    println!("{:?}", pair.as_str());
                    // println!("Text Inner : {:#?}", pair.clone().into_inner());
                    output_string.push_str(pair.as_str());
                    // output_string.push_str(&pair.as_span().as_str().to_string());
                }
                _default => {}
            }
    
            let v = pair.into_inner();
            match MdParser::parse_syntax_tree(v, global_variables, local_variables) {
                Some(parsed) => output_string.push_str(&parsed),
                None => return None,
            }
        }
        return Some(output_string);
    }

}


