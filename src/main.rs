use std::env; // command-line arguments 
use std::fs; // access to functions to loading files

fn main() {
    // get CLI args
    let args: Vec<String> = env::args().collect(); // returning an iterator over command-line
                                                   // args is owned 

    if args.len() < 2 {
        eprintln!("Usage: rusty_editor <filename>"); // standard error (separate from normal output)
        std::process::exit(1); // ends the program with status code 1 (error) immediately
    }

    let filename = &args[1]; // referencing since args is owned and one argument
                                      // cannot be moved to another owner
    match fs::read_to_string(filename) { // reads entire content into string
        Ok(contents) => {             // file read successfully
            println!("--- Contents of {} ---", filename);
            println!("{}", contents);
        }
        Err(error) => {     // something went wrong
            eprintln!("Error reading {}: {}", filename, error);
            std::process::exit(1);
        }
    }
}
