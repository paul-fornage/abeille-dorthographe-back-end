use crate::comb::Comb;
use crate::utils::get_point_value;
use crate::valid_word::ValidWord;
use crate::word_list::WordList;

pub struct Game{
    comb: Comb,
    pub valid_words: Vec<ValidWord>
}

impl Game{
    pub fn new(comb: Comb, word_list: WordList) -> Game{
        Game{
            valid_words: comb.get_valid_words(word_list),
            comb,
        }
    }
    
    pub fn get_total_possible_words(&self) -> usize{
        self.valid_words.len()
    }
    
    pub fn get_possible_points(&self) -> u64{
        self.valid_words.iter().map(|valid_word| {
            get_point_value(&valid_word.word)
        }).sum()
    }
}



