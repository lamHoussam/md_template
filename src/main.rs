use pest_derive::Parser;
use pest::Parser;
use std::fs;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MdParser;

fn main() {
    let input = fs::read_to_string("test/sample.md").expect("Failed to read the file.");

    match MdParser::parse(Rule::start, &input) {
        Ok(parsed) => {
            for pair in parsed {
                println!("Rule: {:#?}, Span: {:#?}", pair.as_rule(), pair.as_span());
            }
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }    
}