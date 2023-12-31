use std::collections::HashMap;
use pest::iterators::{Pairs, Pair};
use crate::{Symbol, get_symbol_from_variable_value};

use std::result::Result;



#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct MdParser;

impl MdParser {

    pub fn parse_for_statement(node: Pair<'_, Rule>, global_variables: &mut HashMap<String, Symbol>, local_variables: &mut HashMap<String, Symbol>) -> Result<String, &'static str> {
        let mut final_string: String = String::new();
        let iter: Pair<'_, Rule> = node;
        let mut cloned_lst: Vec<Symbol>;

        let mut value_iterator_option: Option<std::slice::Iter<'_, Symbol>> = None;
        let mut iter_var_name: String = String::new();

        for pair in iter.into_inner() {
            // println!("Rule: {:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::variable => {
                    iter_var_name = pair.as_str().to_string();
                    // println!("Variable: {}", iter_var_name);
                }
                Rule::variable_value => {
                    // println!("Value: {}", pair.as_str());
                    match pair.into_inner().peek() {
                        Some(iterable_name_pair) => {
                            
                            let iterable_name: &str = iterable_name_pair.as_str();
                            // println!("iterable name: {}", iterable_name);


                            // TODO: Refactor
                            if let Some(value) = local_variables.get(&iterable_name.to_string()).cloned() {
                                match value {
                                    Symbol::String(_) => {
                                        return Err("String is not iterable (Maybe later)")
                                    },
                                    Symbol::Integer(_) => return Err("Integer Not iterable"),
                                    Symbol::Boolean(_) => return Err("Boolean Not iterable"),
                                    Symbol::Struct(_) => return Err("Struct Not iterable"),
                                    Symbol::List(lst) => {
                                        cloned_lst = lst.clone();
                                        value_iterator_option = Some(cloned_lst.iter());
                                    },
                                }
                            } else if let Some(value) = global_variables.get_mut(&iterable_name.to_string()).cloned() {
                                match value {
                                    Symbol::String(_) => {
                                        return Err("String is not iterable (Maybe later)")
                                    },
                                    Symbol::Integer(_) => return Err("Integer Not iterable"),
                                    Symbol::Boolean(_) => return Err("Boolean Not iterable"),
                                    Symbol::Struct(_) => return Err("Struct Not iterable"),
                                    Symbol::List(lst) => {
                                        cloned_lst = lst.clone();
                                        value_iterator_option = Some(cloned_lst.iter());
                                    },
                                }
                            } else {
                                return Err("Couldn't find variable");
                            }

                        } ,
                        None => value_iterator_option = None,
                    }

                },
                Rule::list_litteral => {
                    // println!("Found list litteral: {}", pair.as_str());
                    // return Err("No list litteral");

                    let lst = get_symbol_from_variable_value(pair.as_str().to_string());
                    if let Symbol::List(value) = lst {
                        cloned_lst = value.clone();
                        value_iterator_option = Some(cloned_lst.iter());
                    } else {
                        return Err("Not a list litteral");
                    } 
                }
                ,
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
                            Ok(output) => {
                                final_string.push_str(&output);
                                // println!("{}", output);
                            } 
                            Err(e) => return Err(e),
                        }

                    }
                },

                _default => {

                }
            }

            // println!("Pair: {:#?}", pair.as_str());
        }

        local_variables.remove(&iter_var_name);
        // println!("Final: {}", final_string);
        return Ok(final_string);
    }

    pub fn parse_string_expression(node: Pair<'_, Rule>, global_variables: &mut HashMap<String, Symbol>, local_variables: &mut HashMap<String, Symbol>) -> String {
        let mut final_string: String = String::new();
        match node.as_rule() {
            Rule::variable => {
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
    local_variables: &mut HashMap<String, Symbol>) -> Result<String, &'static str> {
        let iter: Pairs<'_, Rule> = node.clone();
        let mut output_string: String = String::new();

        for pair in iter {
            // println!("Rule: {:#?}", pair.as_rule());
            match pair.as_rule() {
                Rule::assignment_expression => {
                    let assignment_exp: &str = pair.as_str();
                    let vars: Vec<&str> = assignment_exp.split(":=").collect();
                    if vars.len() != 2 {
                        return Err("Error in variable declaration");
                    }

                    let var_name: String = String::from(vars[0].trim());
                    let var_value: String = String::from(vars[1].trim());

                    // println!("Variable: {:?}, Value: {:?}", var_name, var_value);

                    let symb: Symbol = get_symbol_from_variable_value(var_value);

                    global_variables.insert(var_name, symb);
                }
                Rule::print_statement => {
                    // println!("Print Expression: {:#?}", pair.clone().tokens());
                    let node: Pairs<'_, Rule> = pair.clone().into_inner();
                    match node.peek() {
                        Some(expression) => {
                            let mut printed_expression: String = MdParser::parse_string_expression(expression, global_variables, local_variables);                        
                            
                            printed_expression.push('\n');
                            output_string.push_str(&printed_expression);
                        },
                        None => println!("Empty"),
                    }
                }
                Rule::txt => {
                    // println!("{:?}", pair.as_str());
                    // println!("Text Inner : {:#?}", pair.clone().into_inner());
                    output_string.push_str(pair.as_str());
                }
                Rule::for_statement => {
                    let output = match MdParser::parse_for_statement(pair.clone(), global_variables, local_variables) {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    }; 
                    // println!("Foor output: {}", output);
                    
                    output_string.push_str(&output);
                }
                Rule::if_statement => {
                    let output = match MdParser::parse_if_statement(pair.clone(), global_variables, local_variables) {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    }; 
                    
                    output_string.push_str(&output);

                }
                _default => {

                }
            }

            if pair.as_rule() != Rule::for_statement && pair.as_rule() != Rule::if_statement {
                let v: &Pairs<'_, Rule> = &pair.into_inner();
                match MdParser::parse_syntax_tree(v, global_variables, local_variables) {
                    Ok(parsed) => output_string.push_str(&parsed),
                    Err(e) => return Err(e),
                }
            }
    
        }
        return Ok(output_string);
    }
    
    fn evaluate_boolean_expr(node: Pair<'_, Rule>) -> bool {
        match node.as_rule() {
            Rule::boolean_expr => {
                match node.into_inner().peek() {
                    Some(pair) => {
                        return Self::evaluate_boolean_expr(pair);
                    },
                    None => return false,
                }
            },
            Rule::and_expr => {
                let mut val: bool = true;
                for p in node.into_inner() {
                    val &= Self::evaluate_boolean_expr(p);
                    if !val { return false; }
                }

                return val;
            },
            Rule::or_expr => {
                let mut val: bool = false;
                // println!("OR Inner: {:#?}", node.into_inner());
                for p in node.into_inner() {
                    val |= Self::evaluate_boolean_expr(p);
                    if val { return true; }
                }

                return val;
            },
            Rule::not_expr => {
                let mut val: bool = true;
                let mut num_not: i32 = 0;
                for bool_term_pair in node.into_inner() {
                    match bool_term_pair.as_rule() {
                        Rule::NOT => {
                            num_not += 1;
                        }, 
                        Rule::boolean_atom => {
                            val = Self::evaluate_boolean_expr(bool_term_pair);
                        }, 
                        _ => {}
                    }
                }
                return (num_not % 2) == if val { 0 } else { 1 };
            },
            Rule::boolean_atom => {
                match node.into_inner().peek() {
                    Some(pair) => {
                        return Self::evaluate_boolean_expr(pair);
                    },
                    None => return false,
                }
            },
            Rule::boolean_literal => {
                match node.into_inner().peek() {
                    Some(pair) => {
                        let val = pair.as_str().to_string();
                        let symb = get_symbol_from_variable_value(val);

                        return if let Symbol::Boolean(v) = symb { v } else { false }
                    },
                    None => return false,
                }
            },
            _ => {
                // println!("Other: {:?}", node.as_rule());
            }
        }
        return false;
    }

    pub fn parse_if_statement(node: Pair<'_, Rule>, global_variables: &mut HashMap<String, Symbol>, local_variables: &mut HashMap<String, Symbol>) -> Result<String, &'static str> {
        let mut final_string: String = String::new();
        let mut condition_evaluation: bool = false;
        for pair in node.into_inner() {
            // println!("Rule: {:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::boolean_expr => {
                    // println!("Bool expr: {}", pair.as_str());
                    condition_evaluation = Self::evaluate_boolean_expr(pair);
                    // println!("Evaluated to: {}\n", condition_evaluation);
                },
                Rule::variable_value => {
                    let var_name = pair.as_str().replace("$(", "").replace(")", "");
                    if let Some(value) = local_variables.get(&var_name) {
                        match value {
                            Symbol::Boolean(bool_val) => condition_evaluation = *bool_val,
                            _ => return Err("Variable needs to be boolean"),
                        }
                    } else if let Some(value) = global_variables.get(&var_name) {
                        match value {
                            Symbol::Boolean(bool_val) => condition_evaluation = *bool_val,
                            _ => return Err("Variable needs to be boolean"),
                        }
                    } else {
                        return Err("Couldn't find variable");
                    }
                },
                Rule::expression_list => {
                    if condition_evaluation {
                        // println!("expr list: {}, cond: {}", pair.as_str(), condition_evaluation);
                        match MdParser::parse_syntax_tree(&pair.into_inner(), global_variables, local_variables) {
                            Ok(output) => final_string.push_str(&output),
                            Err(e) => return Err(e),
                        }
                    }
                }, 
                Rule::ELSE => {
                    condition_evaluation = !condition_evaluation;
                },
                _ => {

                }
            }
        }

        return Ok(final_string);
    }

}


