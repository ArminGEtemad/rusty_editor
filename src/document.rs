use std::fs;
use std::io;

pub struct Document {
    pub lines: Vec<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(filename)?; 
        let lines = contents
            .lines() 
            .map(|line| line.to_string()) 
            .collect();

        Ok(Self { lines })
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}


