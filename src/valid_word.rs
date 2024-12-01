use std::fmt::Formatter;
use crate::utils::get_point_value;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ValidWord{
    pub word: String,
    pub is_found: bool,
    pub is_panagram: bool,
    pub point_value: u64
}

impl ValidWord{
    pub fn new_unfound(word: String, is_panagram: bool) -> ValidWord{
        ValidWord {
            point_value: get_point_value(&word, is_panagram),
            word,
            is_found: false,
            is_panagram,
        }
    }
}

impl std::fmt::Debug for ValidWord{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match(self.is_panagram, self.is_found){
            (true, true) => {write!(f, "{} - ☑ - Panagram!", self.word)},
            (true, false) => {write!(f, "{} - ☐ - Panagram!", self.word)},
            (false, true) => {write!(f, "{} - ☑", self.word)},
            (false, false) => {write!(f, "{} - ☐", self.word)},
        }
    }
}