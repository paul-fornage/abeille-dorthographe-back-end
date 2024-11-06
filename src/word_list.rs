use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io;
use std::path::Path;

pub struct WordList(pub(crate) Vec<String>);

impl WordList {
    pub fn try_from_file<P>(filename: P) -> Result<Self, io::Error>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;

        Ok(WordList(
            BufReader::new(file).lines().flatten().collect::<Vec<String>>()
        ))
    }
}

