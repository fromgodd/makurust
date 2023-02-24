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
      body {
    font: 400 16px/1.5 "Helvetica Neue", Helvetica, Arial, sans-serif;
    color: #111;
    background-color: #fdfdfd;
    -webkit-text-size-adjust: 100%;
    -webkit-font-feature-settings: "kern" 1;
    -moz-font-feature-settings: "kern" 1;
    -o-font-feature-settings: "kern" 1;
    font-feature-settings: "kern" 1;
    font-kerning: normal;
    padding: 30px;
}

@media only screen and (max-width: 600px) {
    body {
        padding: 5px;
    }

    body > #content {
        padding: 0px 20px 20px 20px !important;
    }
}

body > #content {
    margin: 0px;
    max-width: 900px;
    border: 1px solid #e1e4e8;
    padding: 10px 40px;
    padding-bottom: 20px;
    border-radius: 15px;
    margin-left: auto;
    margin-right: auto;
}

hr {
    color: #bbb;
    background-color: #bbb;
    height: 1px;
    flex: 0 1 auto;
    margin: 1em 0;
    padding: 0;
    border: none;
}

a {
    color: #0366d6;
    text-decoration: none;
}

a:visited {
    color: #0366d6;
}

a:hover {
    color: #0366d6;
    text-decoration: underline;
}

pre {
    background-color: #f6f8fa;
    border-radius: 3px;
    font-size: 85%;
    line-height: 1.45;
    overflow: auto;
    padding: 16px;
}

code {
    background-color: rgba(27,31,35,.05);
    border-radius: 3px;
    font-size: 85%;
    margin: 0;
    word-wrap: break-word;
    padding: .2em .4em;
    font-family: SFMono-Regular,Consolas,Liberation Mono,Menlo,Courier,monospace;
}

pre > code {
    background-color: transparent;
    border: 0;
    display: inline;
    line-height: inherit;
    margin: 0;
    overflow: visible;
    padding: 0;
    word-wrap: normal;
    font-size: 100%;
}

blockquote {
    margin-left: 30px;
    margin-top: 0px;
    margin-bottom: 16px;
    border-left-width: 3px;
    padding: 0 1em;
    color: #828282;
    border-left: 4px solid #e8e8e8;
    padding-left: 15px;
    font-size: 18px;
    letter-spacing: -1px;
    font-style: italic;
}

blockquote * {
    font-style: normal !important;
    letter-spacing: 0;
    color: #6a737d !important;
}

table {
    border-spacing: 2px;
    display: block;
    font-size: 14px;
    overflow: auto;
    width: 100%;
    margin-bottom: 16px;
    border-spacing: 0;
    border-collapse: collapse;
}

td {
    padding: 6px 13px;
    border: 1px solid #dfe2e5;
}

th {
    font-weight: 600;
    padding: 6px 13px;
    border: 1px solid #dfe2e5;
}

tr {
    background-color: #fff;
    border-top: 1px solid #c6cbd1;
}

table tr:nth-child(2n) {
    background-color: #f6f8fa;
}

img {
    max-width: 100%;
}

p {
    line-height: 24px;
    font-weight: 400;
    font-size: 16px;
    color: #24292e;
}

ul {
    margin-top: 0;
}

li {
    color: #24292e;
    font-size: 16px;
    font-weight: 400;
    line-height: 1.5;
}

li + li {
    margin-top: 0.25em;
}

* {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
    color: #24292e;
}

a:visited {
    color: #0366d6;
}

h1, h2, h3 {
    border-bottom: 1px solid #eaecef;
    color: #111;
}
    </style>
  </body>
</html>"#;

    // generate final html
    let output_filename = input_filename.replace(".md", ".html");
    let mut output_file = File::create(output_filename)?;
    write!(output_file, "{}{}{}", precontent, html_output, postcontent)?;

    Ok(())
}
