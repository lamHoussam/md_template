use pest::iterators::Pairs;
use pest_derive::Parser;
use pest::Parser;
use std::collections::HashMap;
use std::fs;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MdParser;

enum Symbol {
    String(String),
    Integer(i32),
    Boolean(bool),
    Struct(HashMap<String, Symbol>),
}


fn parse_syntax_tree(node: Pairs<'_, Rule, >, mut variables: &HashMap<String, Symbol>) {
    let mut iter = node;
    for pair in iter {
        println!("Rule: {:#?}, Value: {:#?}", pair.as_rule(), pair.as_str());

        let v = pair.into_inner();
        parse_syntax_tree(v, variables);
    }
}

fn main() {
    let data_file_path: &str = "data/data.txt";
    let sample_file_path: &str = "test/sample.md";

    let data_file: String = fs::read_to_string(data_file_path).expect("Failed to read data file");
    let sample_file: String = fs::read_to_string(sample_file_path).expect("Failed to read sample file.");
    
    let variables: HashMap<String, Symbol> = HashMap::new();

    println!("Data file: ");
    match MdParser::parse(Rule::start, &data_file) {
        Ok(parsed) => {
            parse_syntax_tree(parsed, &variables);
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }

    println!("Sample file: ");
    match MdParser::parse(Rule::start, &sample_file) {
        Ok(parsed) => {
            for pair in parsed {
                println!("Rule: {:#?}, Span: {:#?}", pair.as_rule(), pair.as_span());
            }
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }
}