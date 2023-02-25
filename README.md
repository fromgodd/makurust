<h1 align="center">Makurust</h1>

<p align="center">
<img src="https://user-images.githubusercontent.com/97128346/221287390-bb500651-5daa-4fbc-a1f4-10f34fc27ff8.png" width="300px" height="300px">
</p>
<h3 align="center">Makurust is a powerful tool written in Rust that allows you to effortlessly convert your Markdown files into static HTML pages. Inspired by Typescript based Makudaun tool that was originially written by Yuri Katsuki. Makurust is designed to make the conversion process as fast and efficient as possible.</h3>
<p>


## Usage
To use Makurust, simply run the following command:
```bash
chmod a+x ./makurust
./makurust filename.md
```
As a result, an HTML page will be created from the given Markdown file. If the output file already exists, it will be overwritten.<br>

Makurust was build using third party library - `pulldown-cmark = "0.9.2"`
## Building from source
1. Clone the Makurust repository using Git:

```bash
git clone https://github.com/saidofficial/makurust
```

2. Navigate to the Makurust directory
```bash
cd makurust
```

3. Use Cargo to compile the program with optional '--release' flag
```bash
cargo build --release
```

4. Run using Cargo run or immediately use binary
- Cargo run
```bash
cargo run
```

- Using binary after build
```bash
cd target/release/
./makurust
```


### What is working
- [X] Basic Markdown Support
- [x] GitHub-like Markdown Style
- [x] Aligning
- [x] Images
- [x] Code snippets
### Under development
- [ ] Text style syntax (Italic, Underlined, Stroke Through)
- [ ] Table
- [ ] Syntax highlighting for snippets
- [ ] Checkboxes

### TODO: Implement Table support, ~~Implement panics~~, OS System Err and etc.


