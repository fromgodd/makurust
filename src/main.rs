use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // cmd line arguments
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    // read contents of given arguments
    let mut input_file = File::open(input_filename)?;
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents)?;

    // parse md -> html
    let options = Options::empty();
    let parser = Parser::new_ext(&input_contents, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Generate the precontent and postcontent (BIG THANKS TO YURI KATSUKI)
    let precontent = format!(
        r#"<html lang="en">
  <head>
    <title>{}</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
  </head>
  <body>
    <div id='content'>"#,
        input_filename.replace(".md", "")
    );
    let postcontent = r#"
    </div>
    <style>
      /* Your CSS styles here */
    </style>
  </body>
</html>"#;

    // generate final html
    let output_filename = input_filename.replace(".md", ".html");
    let mut output_file = File::create(output_filename)?;
    write!(output_file, "{}{}{}", precontent, html_output, postcontent)?;

    Ok(())
}
