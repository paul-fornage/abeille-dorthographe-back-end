use crate::comb::Comb;
use crate::word_list::WordList;

mod comb;
mod word_list;
mod utils;
mod game;
mod valid_word;

fn main() {
    let fr_word_list = WordList::try_from_file("fr_word_list.txt").unwrap();
    let comb = Comb::new_random(&fr_word_list);
    println!("Comb: {:#?}", comb);
    let valid_words = comb.get_valid_words(fr_word_list);
    println!("Valid words: {:#?}", valid_words);
}
