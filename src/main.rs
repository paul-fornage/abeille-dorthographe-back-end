use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::comb::Comb;
use crate::game::{Game, GameIdentifier};
use crate::word_list::WordList;
use dotenv::dotenv;
use anyhow::{Context, Error, Result};
use couch_rs::database::Database;
use couch_rs::document::DocumentCollection;
use couch_rs::error::{CouchError, CouchResult};
use couch_rs::http::StatusCode;
use couch_rs::types::find::FindQuery;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, Response};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::{Debug, Redirect};
use rocket::{get, routes};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde_json::{self, Value};
use lazy_static::lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref DB_CLIENT: couch_rs::Client = {
        dotenv().ok();
        let db_host: &str = &env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");
        let db_username: &str = &env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set in env");
        let db_password: &str = &env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set in env");

        let client = couch_rs::Client::new(db_host, db_username, db_password).expect("Error while connecting to database");
        client
    };
    
    static ref LANG_LIST: LangList = {
        let file = File::open("languages/languages.json").expect("Error while opening lang list file");
        let langs: Vec<LanguageCode> = serde_json::from_str(BufReader::new(file).lines().flatten().collect::<String>().as_str()).expect("Error while parsing lang list file");
        LangList(langs)
    };
    
    static ref WORD_LISTS: Vec<WordList> = {
        
        LANG_LIST.0.iter().map(|lang| {
            let path = format!("languages/{}.txt", lang.code);
            println!("Loading word list for language {} from {}", lang.code, path);
            WordList::try_from_file(path, lang.clone()).expect("Error while loading word list")
        }).collect::<Vec<WordList>>()
    };
}


mod comb;
mod word_list;
mod utils;
mod game;
mod valid_word;
mod user;
mod lang;
mod typo;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use crate::lang::{LangList, LanguageCode};
use crate::typo::Typo;
use crate::typo::Typo::CouchDbError;
use crate::user::User;
use crate::utils::get_local_date;

#[macro_use]
extern crate rocket;
extern crate rocket_cors;


fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("error while building CORS")
}




#[get("/api/<lang_code>/wordlist")]
async fn get_wordlist_for_lang(lang_code: &str) -> String {
    match WORD_LISTS.iter().find(|word_list| word_list.language_code.code == lang_code)
    {
        Some(word_list) => {
            word_list.to_string()
        }
        None => {
            "Not implemented yet".to_string()
        }
    }
}

#[get("/api/langs")]
async fn get_supported_langs() -> String {
    LANG_LIST.to_string()
}

#[get("/api/available/games")]
async fn get_available_games() -> String {
    
    let db = DB_CLIENT.db("daily-games").await.expect("Error while connecting to database");

    let get_dates_and_games = FindQuery::find_all()
        .fields(Vec::from([
            "date".to_string(), 
            "language_code".to_string()
        ]));

    let idents = db.find_raw(&get_dates_and_games).await
        .expect("Error while loading games");

    serde_json::to_string(idents.get_data()).expect("Error while serializing game")
}


async fn get_todays_game(lang_code: &str) -> Option<String> {
    let date = get_local_date().await;
    match get_daily_game(lang_code, &date.to_string()).await {
        Ok(game) => {
            Some(serde_json::to_string(&game).expect("Error while serializing game"))
        }
        Err(Typo::GetDailyGameNotFound()) => {
            let word_list = match WORD_LISTS.iter().find(|word_list| word_list.language_code.code == lang_code) {
                Some(word_list) => {
                    word_list
                }
                None => {
                    warn!("get_today's game request failed: Language not implemented yet");
                    return None
                }
            };
            let db = DB_CLIENT.db("daily-games").await.expect("Error while connecting to database");
            let mut game = Game::new_daily_game(word_list).await;
            db.save(&mut game).await.expect("Error while saving new daily game");
            Some(serde_json::to_string(&game).expect("Error while serializing game"))
        }
        Err(other_error) => {
            warn!("Unable to get today's daily game. \nlang_code: {}, \nerror: {}", lang_code, other_error);
            None
        }
    }
}

#[get("/api/<lang_code>/dailygame/<date>")]
async fn request_get_daily_game(lang_code: &str, date: &str) -> Option<String> {
    if date == "today" {
        return get_todays_game(lang_code).await;
    }
    match get_daily_game(lang_code, date).await {
        Ok(game) => {
            Some(serde_json::to_string(&game).expect("Error while serializing game"))
        }
        Err(typo) => {
            warn!("Unable to get daily game. \ndate: {} \nlang_code: {}, \nerror: {}", date, lang_code, typo);
            None
        }
    }
}
async fn get_daily_game(lang_code: &str, date: &str) -> Result<Game, Typo> {

    let db = DB_CLIENT.db("daily-games").await.expect("Error while connecting to database");
    
    let find_by_date = FindQuery::new(serde_json::json!({"$and":[{"date": date}, {"language_code.code": lang_code}]})).limit(1);

    match db.find::<Game>(&find_by_date).await {
        Ok(doc) => {
            let game_option = doc.rows.into_iter().next();
            game_option.ok_or(Typo::GetDailyGameNotFound())
        }
        Err(CouchError::OperationFailed(error_details))
            if error_details.status == StatusCode::CONFLICT => {
            Err(Typo::GetDailyGameNotFound())
            // skip this path and continue to doc creation. This seems to be returned when one of
            // the conditions are met, but no docs match all conditions.
            // e.g. None found, 'but I did find yesterdays english game'
        }
        Err(other_couch_error) => {
            warn!("Error while loading game");
            Err(CouchDbError(other_couch_error))
        }
    }
}




#[rocket::main]
async fn main() -> Result<()> {



    rocket::build()
        .mount("/", routes![
            get_wordlist_for_lang,
            get_supported_langs,
            request_get_daily_game,
            get_available_games,
        ])
        .attach(make_cors())
        .launch()
        .await?;

    Ok(())
}


#[cfg(test)]
mod comb_tests {
    use super::*;

    #[rocket::async_test]
    async fn run_sample_game_generation() {
        let word_list = WORD_LISTS.iter().find(|word_list| word_list.language_code.code == "fr").expect("Language not implemented yet");
        let game = Game::new_daily_game(word_list).await;
        println!("Valid words: {:#?}", game.valid_words);

        println!("Game has {} possible words worth a total of {} points.", game.total_words, game.total_points);
    }

    #[rocket::async_test]
    async fn test_get_available_games() {
        let games_str = get_available_games().await;
        println!("available_games:\n{:#?}", games_str);
    }
}