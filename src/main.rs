mod md_lexer;
mod md_parser;

// use crate::md_parser::{MdParser, Rule};
use std::time::{Instant, Duration};
use std::fs::{self, File};

use clap::Parser as ClapParser;
use md_lexer::MdLexer;

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    /// Data file path for global variables
    // TODO: Read from json later
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
    
    let sample_file_path = args.sample.as_str();
    // let output_file_path = args.output.as_str();
    
    let sample_file_content = fs::read_to_string(sample_file_path).expect("Failed to read sample file.");

    let mut start = Instant::now();
    println!("Start Lexer");
    let mut scanner = MdLexer::new(&sample_file_content);
    scanner.scan_tokens();
    let mut end = Instant::now();
    let mut duration = end - start;
    println!("Lexer execution time: {:?}", duration);

    println!("Start Parser");
    start = Instant::now();
    let mut parser = md_parser::MdParser::new(&mut scanner.tokens);
    parser.parse();
    end = Instant::now();
    duration = end - start;
    println!("Parser execution time {:?}", duration);

    // println!("{:#?}", scanner.tokens);


    // let mut output_file = match File::create(output_file_path) {
    //     Ok(file) => file,
    //     Err(_) => panic!("Couldn't create output file"),
    // };

    // let data_file_path = args.data.unwrap_or_default();
    // let data_file_content = fs::read_to_string(data_file_path).unwrap_or_default();
}