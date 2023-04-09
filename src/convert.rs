use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*};
use std::error::Error;

pub fn markdown_to_html(input_contents: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(input_contents, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn write_to_file(html_output: String, output_filename: &str) -> Result<(), Box<dyn Error>> {
    let precontent = format!(
        r#"<html lang="en">
        <head>
            <title>{}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <style>
                {}
            </style>
        </head>
        <body>
            <div id='content'>"#,
        output_filename.replace(".md", ""),
        fs::read_to_string("styles/style.css")?,
    );
    let postcontent = r#"
            </div>
        </body>
    </html>"#;
    let mut output_file = File::create(output_filename)?;
    write!(output_file, "{}{}{}", precontent, html_output, postcontent)?;
    Ok(())
}
