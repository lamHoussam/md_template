use std::collections::HashMap;
use pest::iterators::{Pairs, Pair};
use crate::{Symbol, get_symbol_from_variable_value};



#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct MdParser;

impl MdParser {

    pub fn parse_for_statement(node: Pair<'_, Rule>, global_variables: &mut HashMap<String, Symbol>, local_variables: &mut HashMap<String, Symbol>) -> String {
        let mut final_string: String = String::new();
        let iter: Pair<'_, Rule> = node;
        let mut cloned_lst: Vec<Symbol>;

        let mut value_iterator_option: Option<std::slice::Iter<'_, Symbol>> = None;
        let mut iter_var_name: String = String::new();

        for pair in iter.into_inner() {
            println!("Rule: {:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::variable => {
                    iter_var_name = pair.as_str().to_string();
                    println!("Variable: {}", iter_var_name);
                }
                Rule::variable_value => {
                    println!("Value: {}", pair.as_str());
                    match pair.into_inner().peek() {
                        Some(iterable_name_pair) => {
                            
                            let iterable_name: &str = iterable_name_pair.as_str();
                            println!("iterable name: {}", iterable_name);


                            // TODO: Refactor
                            value_iterator_option = None;
                            if let Some(value) = local_variables.get(&iterable_name.to_string()).cloned() {
                                match value {
                                    Symbol::String(_) => todo!(),
                                    Symbol::Integer(_) => todo!(),
                                    Symbol::Boolean(_) => todo!(),
                                    Symbol::Struct(_) => todo!(),
                                    Symbol::List(lst) => {
                                        cloned_lst = lst.clone();
                                        value_iterator_option = Some(cloned_lst.iter());
                                    },
                                }
                            } else if let Some(value) = global_variables.get_mut(&iterable_name.to_string()).cloned() {
                                match value {
                                    Symbol::String(_) => todo!(),
                                    Symbol::Integer(_) => todo!(),
                                    Symbol::Boolean(_) => todo!(),
                                    Symbol::Struct(_) => todo!(),
                                    Symbol::List(lst) => {
                                        cloned_lst = lst.clone();
                                        value_iterator_option = Some(cloned_lst.iter());
                                    },
                                }
                            }

                        } ,
                        None => value_iterator_option = None,
                    }

                },
                Rule::expression_list => {
                    let node = &pair.into_inner();

                    
                    loop {
                        if let Some(mut val_iterator) = value_iterator_option {
                            match val_iterator.next() {
                                Some(value) => {
                                    local_variables.insert(iter_var_name.to_string(), value.clone());
                                    value_iterator_option = Some(val_iterator);
                                },
                                None => {
                                    value_iterator_option = None;
                                    break;
                                },
                            }
                        }
                        
                        
                        match MdParser::parse_syntax_tree(node, global_variables, local_variables) {
                            Some(output) => {
                                final_string.push_str(&output);
                                println!("{}", output);
                            } 
                            None => todo!(),
                        }

                    }
                },

                _default => {

                }
            }

            // println!("Pair: {:#?}", pair.as_str());
        }

        local_variables.remove(&iter_var_name);
        println!("Final: {}", final_string);
        return final_string;
    }

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
    
    pub fn parse_syntax_tree(node: &Pairs<'_, Rule, >, global_variables: &mut HashMap<String, Symbol>, 
    local_variables: &mut HashMap<String, Symbol>) -> Option<String> {
        let iter: Pairs<'_, Rule> = node.clone();
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
                    // println!("{:?}", pair.as_str());
                    // println!("Text Inner : {:#?}", pair.clone().into_inner());
                    output_string.push_str(pair.as_str());
                }
                Rule::for_statement => {
                    let output = MdParser::parse_for_statement(pair.clone(), global_variables, local_variables);
                    println!("Foor output: {}", output);
                    
                    output_string.push_str(&output);
                }
                _default => {

                }
            }

            if pair.as_rule() != Rule::for_statement {
                let v: &Pairs<'_, Rule> = &pair.into_inner();
                match MdParser::parse_syntax_tree(v, global_variables, local_variables) {
                    Some(parsed) => output_string.push_str(&parsed),
                    None => return None,
                }
            }
    
        }
        return Some(output_string);
    }

}


