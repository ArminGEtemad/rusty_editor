use std::fs;
use std::io; // io needed for input, output errors

// a struct to hold the content
pub struct Document {
    lines: Vec<String>, // each line stored as a separate string
}

// implementing Document method
impl Document {
    pub fn open(filename: &str) -> Result<Self, io::Error> {
                                // If everything goes well the result is Self
                                // If it fails we get io::Error
        let contents = fs::read_to_string(filename)?; // ? give the value if 
                                                      // Result is Ok and stop 
                                                      // right here if Err
        let lines = contents
            .lines() // spliting the file into lines
            .map(|line| line.to_string()) // converts each borrowed &str into owned String
            .collect(); // collecting into a vector

        Ok(Self { lines })
    }
    /*
    pub fn display(&self) {
        for (i, line) in self.lines.iter().enumerate() {
            println!("{:>4} | {}", i + 1, line);
        }
    }
     */

    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}


