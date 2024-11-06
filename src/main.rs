use crate::comb::Comb;
use crate::game::Game;
use crate::word_list::WordList;

mod comb;
mod word_list;
mod utils;
mod game;
mod valid_word;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

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


#[get("/")]
fn index() -> &'static str {
    "Hello, world! Cock and ball torture!!!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).attach(make_cors()) // 7.
}


#[cfg(test)]
mod comb_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn run_sample_game_generation() {
        let fr_word_list = WordList::try_from_file("fr_word_list.txt").unwrap();
        let comb = Comb::new_random(&fr_word_list);
        println!("Comb: {:#?}", comb);
        let game = Game::new(comb, fr_word_list);
        println!("Valid words: {:#?}", game.valid_words);

        println!("Game has {} possible words worth a total of {} points.", game.get_total_possible_words(), game.get_possible_points());
    }
}