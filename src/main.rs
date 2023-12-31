use pest::Parser as PestParser;

pub mod symbol;
pub mod md_parser;

use crate::md_parser::{MdParser, Rule};
use core::panic;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;

use clap::Parser as ClapParser;

pub use symbol::{Symbol, get_symbol_from_variable_value};

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    /// Data file path for global variables
    #[arg(short, long)]
    data: Option<String>,
    /// Sample markdown file
    #[arg(short, long)]
    sample: String,
    /// Output markdown file
    #[arg(short, long)]
    output: String,
}


fn main() {
    
    let args = Args::parse();
    
    // println!("Sample: {}, Output: {}", args.sample, args.output);
    
    let sample_file_path: &str = args.sample.as_str();
    let output_file_path: &str = args.output.as_str();

    let sample_file: String = fs::read_to_string(sample_file_path).expect("Failed to read sample file.");
    
    let mut output_file: File = match File::create(output_file_path) {
        Ok(file) => file,
        Err(_) => panic!("Couldn't create output file"),
    };

    let mut global_variables: HashMap<String, Symbol> = HashMap::new();
    let mut local_variables: HashMap<String, Symbol> = HashMap::new();

    // println!("Data file: ");
    match args.data {
        Some(data_file_path) => {
            let data_file: String = fs::read_to_string(data_file_path).expect("Failed to read data file");

            match MdParser::parse(Rule::start, &data_file) {
                Ok(parsed) => {
                    match MdParser::parse_syntax_tree(&parsed, &mut global_variables, &mut local_variables) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Error while parsing: {:?}", e),
                    };
                }
                Err(e) => eprintln!("Error while parsing: {:?}", e),
            }
        },    
        None => {},
    }    

    match MdParser::parse(Rule::start, &sample_file) {
        Ok(parsed) => {
            // println!("Output {:#?}", parsed);
            match MdParser::parse_syntax_tree(&parsed, &mut global_variables, &mut local_variables) {
                Ok(output) => {
                    match output_file.write_all(output.as_bytes()) {
                        Ok(_) => {},
                        Err(_) => panic!("Couldn't write to output file"),
                    }
                },
                Err(e) => panic!("Couldn't generate output string {e:?}"),
            }
        }
        Err(e) => eprintln!("Error while parsing: {:?}", e),
    }
}