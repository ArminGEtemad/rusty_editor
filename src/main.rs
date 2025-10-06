mod document;
mod screen;
mod editor;

use std::env; // command-line arguments 
use document::Document; // in order to use Document
use editor::Editor;

fn main() -> std::io::Result<()> {
    // get CLI args
    let args: Vec<String> = env::args().collect(); // returning an iterator over command-line
                                                   // args is owned 

    if args.len() < 2 {
        eprintln!("Usage: rusty_editor <filename>"); // standard error (separate from normal output)
        std::process::exit(1); // ends the program with status code 1 (error) immediately
    }

    let filename = &args[1]; // referencing since args is owned and one argument
                                      // cannot be moved to another owner
    let doc = match Document::open(filename) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Error Reading {}, {}", filename, err);
            std::process::exit(1);
        },
    };
    let mut editor = Editor::new(doc, filename.clone());
    editor.run()
}
