use couch_rs::document::TypedCouchDocument;
use chrono::NaiveDate;
use couch_rs::CouchDocument;
use couch_rs::types::document::DocumentId;
use serde::Serializer;
use sha3::Digest;
use crate::comb::Comb;
use crate::utils::{get_local_date, get_point_value};
use crate::valid_word::ValidWord;
use crate::word_list::WordList;

#[derive(serde::Serialize, Debug, serde::Deserialize, CouchDocument)]
pub struct Game{
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub comb: Comb,
    pub date: NaiveDate,
    pub total_words: u64,
    pub total_points: u64,
    pub valid_words: Vec<ValidWord>
}

impl Game{
    

    pub fn new_id(comb: Comb, date: NaiveDate, word_list: &WordList, id: DocumentId) -> Game{
        let valid_words = comb.get_valid_words(word_list);
        let total_points: u64 = valid_words.iter().map(|valid_word| {
            get_point_value(&valid_word.word, valid_word.is_panagram)
        }).sum();
        Game{
            comb,
            date,
            total_words: valid_words.len() as u64,
            _id: id,
            _rev: "".to_string(),
            total_points,
            valid_words,
        }
    }


    


    pub async fn new_daily_game(word_list: &WordList) -> Self{
        
        let comb = Comb::new_random(&word_list);
        let today = get_local_date().await;
        
        let id = base16ct::lower::encode_string(&sha3::Sha3_256::digest(&today.to_string()));

        Game::new_id(comb, today, word_list, id)
    }
}

