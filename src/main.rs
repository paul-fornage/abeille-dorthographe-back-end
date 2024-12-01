use std::env;
use crate::comb::Comb;
use crate::game::Game;
use crate::word_list::WordList;
use dotenv::dotenv;
use anyhow::{Context, Error, Result};
use couch_rs::database::Database;
use couch_rs::document::DocumentCollection;
use couch_rs::error::CouchResult;
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
    
    static ref FR_WORD_LIST: WordList = WordList::try_from_file("fr_word_list.txt").expect("Error while loading word list");
    
    
}


mod comb;
mod word_list;
mod utils;
mod game;
mod valid_word;
mod user;
mod lang;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
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




#[get("/wordlist/<lang_code>")]
async fn get_wordlist_for_lang(lang_code: &str) -> String {
    if lang_code == "fr" {
        FR_WORD_LIST.to_string()
    } else {
        "Not implemented yet".to_string()
    }
}



#[get("/dailygame/today")]
async fn get_todays_game() -> String {
    let db = DB_CLIENT.db("daily-games").await.expect("Error while connecting to database");

    let date = get_local_date().await;
    let find_by_date = FindQuery::new(serde_json::json!({"date": date})).limit(1);
    match db.find::<Game>(&find_by_date).await.expect("Error while finding game").get_data().first() {
        Some(game) => {
            serde_json::to_string(&game).expect("Error while serializing game")
        }
        None => {
            let mut game = Game::new_daily_game(&FR_WORD_LIST).await;
            db.save(&mut game).await.expect("Error while saving new daily game");
            serde_json::to_string(&game).expect("Error while serializing game")
        }
    }
}




#[rocket::main]
async fn main() -> Result<()> {



    rocket::build()
        .mount("/", routes![
            get_todays_game,
            get_wordlist_for_lang,
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
        let game = Game::new_daily_game(&FR_WORD_LIST).await;
        println!("Valid words: {:#?}", game.valid_words);

        println!("Game has {} possible words worth a total of {} points.", game.total_words, game.total_points);
    }
}