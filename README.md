<h1 align="center">Makurust</h1>

Markdown to HTML converter written in Rust. Inspired by [Katsuki Yuri's Makudaun Tool](https://github.com/katsuki-yuri/makudaun)
<p>
Special thanks to Yuri Katsuki for providing help in the project</p>

## Usage
To use Makurust, simply run the following command:
```bash
makurust <input-file.md>
```
As a result, an HTML page will be created from the given Markdown file. If the output file already exists, it will be overwritten.

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
<p>Dependency used 'pulldown-cmark = "0.9.2'</p>

### What is working
- [x] GitHub-like Markdown Style
- [x] Images
- [x] Aligning
- [ ] Table


### TODO: Implement Table support, Implement panics, backtrace and etc.


