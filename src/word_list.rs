use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io;
use std::path::Path;
use crate::lang::LanguageCode;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WordList {
    pub words: Vec<String>,
    pub language_code: LanguageCode,
}

impl WordList {
    pub fn try_from_file<P>(filename: P, language_code: LanguageCode) -> Result<Self, io::Error>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;

        Ok(WordList{
            words: BufReader::new(file).lines().flatten().collect::<Vec<String>>(),
            language_code,
        })
    }
    pub fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
    
    
}

