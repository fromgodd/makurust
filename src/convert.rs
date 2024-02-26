use pulldown_cmark::{html, Options, Parser};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn markdown_to_html(input_contents: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(input_contents, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn write_to_file(html_output: String, output_filename: &str) -> Result<(), Box<dyn Error>> {
    // If we are in the git repo or someone has provided a css file near their working directory we can use that.
    // Otherwise use the one included at compile time.
    let css = if Path::new("styles/style.css").exists() {
        fs::read_to_string("styles/style.css")?
    } else {
        DEFAULT_CSS.to_string()
    };

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
        css,
    );
    let postcontent = r#"
            </div>
        </body>
    </html>"#;
    let mut output_file = File::create(output_filename)?;
    write!(output_file, "{}{}{}", precontent, html_output, postcontent)?;
    Ok(())
}

// Include the default css at compile time so it doesn't matter where the binary ends up it will always be able to find
// this file.
const DEFAULT_CSS: &str = include_str!("../styles/style.css");
