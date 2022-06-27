use std::fmt::Display;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

pub struct ResultsWriter {
    file : File,
}

impl ResultsWriter {
    pub fn create(name : &str) -> Result<Self, Error> {
        let mut output_path = PathBuf::from("results").join(name);
        output_path.set_extension("csv");
        let output = File::create(output_path)?;
        Ok(Self {file : output})
    }
    pub fn add_row<T : Display>(&mut self, row_values : Vec<T>) -> Result<(),Error> {
        let mut row_string = String::new();
        for value in row_values {
            row_string = row_string + &format!("{},", value);
        }
        row_string.pop();
        writeln!(self.file, "{}", row_string)?;
        Ok(())
    }
}