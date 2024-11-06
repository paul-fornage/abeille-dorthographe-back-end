use rand::Rng;
use crate::utils::{get_unique_letters};
use crate::valid_word::ValidWord;
use crate::word_list::WordList;

#[derive(Debug)]
pub struct Comb {
    center_char: char,
    outer_chars: [char; 6],
}

impl Comb {
    pub fn new_random(word_list: &WordList) -> Comb {
        let num_words: usize = word_list.0.len();
        let starting_index: usize = rand::thread_rng().gen_range(0..num_words);
        let mut index: usize = starting_index;

        let mut current_word = &word_list.0[index];
        let mut current_unique_chars = get_unique_letters(current_word);

        while current_unique_chars.len() != 7 {
            index += 1;
            current_word = &word_list.0[index];
            current_unique_chars = get_unique_letters(current_word);
        }
        let center_letter_index: usize = rand::thread_rng().gen_range(0..7);

        println!("found panagram: {current_word}");

        Self{
            center_char: current_unique_chars.remove(center_letter_index),
            outer_chars: <[char; 6]>::try_from(current_unique_chars.as_slice()).unwrap() // unwrap safety: This should be infallible
        }
    }
    
    pub fn check_word_status(&self, word_to_check: &str) -> WordStatus {
        let mut contains_center: bool = false;
        let word_dedup = get_unique_letters(word_to_check);
        for char in &word_dedup{
            if char == &self.center_char {
                contains_center = true;
            } else {
                if !self.outer_chars.contains(char) {
                    return WordStatus::NotValid;
                }
            }
        }
        if !contains_center{
            WordStatus::NotValid
        } else if word_dedup.len() == 7 {
            WordStatus::Panagram
        } else {
            WordStatus::Valid
        }
    }
    
    pub fn get_valid_words(&self, word_list: WordList) -> Vec<ValidWord> {
        word_list.0.iter().filter_map(|word| {
            match self.check_word_status(word){
                WordStatus::Valid => Some(ValidWord::new_unfound(word.clone(), false)),
                WordStatus::Panagram => Some(ValidWord::new_unfound(word.clone(), true)),
                WordStatus::NotValid => None
            }
        }).collect::<Vec<ValidWord>>()
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum WordStatus {
    NotValid,
    Valid,
    Panagram
}


#[cfg(test)]
mod comb_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    

    #[test]
    fn test_check_word_validity() {
        let test_comb = Comb{
            center_char: 'A',
            outer_chars: ['B','C','D','E','F','G'],
        };
        assert_eq!(test_comb.check_word_status("A"), WordStatus::Valid);
        assert_eq!(test_comb.check_word_status("AB"), WordStatus::Valid);
        assert_eq!(test_comb.check_word_status("BAC"), WordStatus::Valid);
        assert_eq!(test_comb.check_word_status("CBGA"), WordStatus::Valid);
        assert_eq!(test_comb.check_word_status("AAAA"), WordStatus::Valid);
        assert_eq!(test_comb.check_word_status(""), WordStatus::NotValid);
        assert_eq!(test_comb.check_word_status("B"), WordStatus::NotValid);
        assert_eq!(test_comb.check_word_status("BCGFD"), WordStatus::NotValid);
        assert_eq!(test_comb.check_word_status("CBGZ"), WordStatus::NotValid);
        assert_eq!(test_comb.check_word_status("AAAAZ"), WordStatus::NotValid);
        assert_eq!(test_comb.check_word_status("ABCDEFG"), WordStatus::Panagram);
        assert_eq!(test_comb.check_word_status("ABCDEFGABCDEFGABCDEFG"), WordStatus::Panagram);
        assert_eq!(test_comb.check_word_status("GFEDCBA"), WordStatus::Panagram);
        assert_eq!(test_comb.check_word_status("GFEDCBAABCDEFG"), WordStatus::Panagram);
        assert_eq!(test_comb.check_word_status("AAAAAAAAABCDEFG"), WordStatus::Panagram);
        
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(1 + 2, 3);
    }
}