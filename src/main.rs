use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use std::env;

mod args;
mod convert;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args()?;
    let input_contents = parse::parse_file(&args.input_filename)?;
    let html_output = convert::markdown_to_html(&input_contents);
    let output_filename = args.input_filename.replace(".md", ".html");
    convert::write_to_file(html_output, &output_filename)?;
    println!("Succesfully converted!");
    Ok(())
}
