use std::collections::HashMap;
use std::fmt::Debug;

pub enum Symbol {
    String(String),
    Integer(i32),
    Boolean(bool),
    Struct(HashMap<String, Symbol>),
    List(Vec<Symbol>)
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").field(arg0).finish(),
            Self::List(arg0) => f.debug_tuple("List").field(arg0).finish(),
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        return match self {
            Symbol::String(value) => String::from(value),
            Symbol::Integer(value) => value.to_string(),
            Symbol::Boolean(value) => value.to_string(),
            Symbol::Struct(value) => todo!(),
            Symbol::List(value) => {
                let iter: std::slice::Iter<'_, Symbol> = value.iter();

                let strings: Vec<String> = iter.map(|x: &Symbol| x.to_string()).collect();
                let result: String = strings.join(", ");

                return result;
            },
        }
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        match self {
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Integer(arg0) => Self::Integer(arg0.clone()),
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Struct(arg0) => Self::Struct(arg0.clone()),
            Self::List(arg0) => Self::List(arg0.clone()),
        }
    }
}

pub fn get_symbol_from_variable_value(var_value: String) -> Symbol {
    if var_value.starts_with('\'') {
        let final_string: &str = var_value.trim_matches('\'');
        return Symbol::String(final_string.to_string());
    } else if var_value.contains("[") {
        let mut list: Vec<Symbol> = Vec::new();
        let string_values: Vec<&str> = var_value.split(",").collect();
        
        for ele in string_values {
            let v: String = ele.replace("[", "").replace("]", "").trim().to_string();

            // println!("v: {}", v);
            if !v.is_empty() {
                let s: Symbol = get_symbol_from_variable_value(v);
                list.push(s);
            }
        }
        return Symbol::List(list);
    } else if var_value.eq("True") {
        return Symbol::Boolean(true);
    } else if var_value.eq("False") {
        return Symbol::Boolean(false);
    } else {
        return Symbol::Integer(var_value.parse().unwrap());
    }
}
