# Rusty Editor
RustyEditor is a minimal terminal-based text editor written in Rust. It is inspired by [this project](https://viewsourcecode.org/snaptoken/kilo/)

## How to use it?
Just like any other super simple editors.
- Ctrl + S to save
- Ctrl + Q to quit

to use it:
```bash
cargo run -- filename.txt
```

## Limitations
- No syntax highlighting
- No tabs, search, or undo
- Limited to small `.txt` files (<10k lines)
- Doesn’t support multi-byte characters yet
