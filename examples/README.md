<h1 align="center">Makurust</h1>

<p align="center">
<img src="https://user-images.githubusercontent.com/97128346/221287390-bb500651-5daa-4fbc-a1f4-10f34fc27ff8.png" width="300px" height="300px">
</p>
Markdown to HTML converter written in Rust. Inspired by Katsuki Yuri's Makudaun Tool
<p>
Special thanks to Yuri Katsuki for providing help in the project</p>

## Usage
To use Makurust, simply run the following command:
```bash
chmod a+x ./makurust
./makurust filename.md
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


### TODO: Implement Table support, ~~Implement panics~~, OS System Err and etc.


