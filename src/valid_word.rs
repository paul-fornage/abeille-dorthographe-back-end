use std::fmt::Formatter;

pub struct ValidWord{
    pub word: String,
    pub is_found: bool,
    pub is_panagram: bool,
}

impl ValidWord{
    pub fn new_unfound(word: String, is_panagram: bool) -> ValidWord{
        ValidWord {
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