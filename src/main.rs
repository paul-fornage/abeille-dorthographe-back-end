use std::env;
use crate::comb::Comb;
use crate::game::Game;
use crate::word_list::WordList;
use rocket::Request;
use dotenv::dotenv;
use anyhow::{Context, Error, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Response;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::request;
use rocket::response::{Debug, Redirect};
use rocket::{get, routes};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde_json::{self, Value};
use sha3::{Digest, Sha3_256};
use sqlx::{postgres::PgPool, *};
use rocket_db_pools::{Database, Connection};

#[derive(Database)]
#[database("postgres_db")]
pub struct Db(sqlx::PgPool);


struct UserID(String);

mod comb;
mod word_list;
mod utils;
mod game;
mod valid_word;
mod user;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use sqlx::postgres::PgPoolOptions;
use crate::user::User;

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

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for UserID {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<UserID, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");

        let user_id_hashed: UserID = match cookies.get_private("user_id_hashed"){
            None => {return request::Outcome::Forward(Status::Unauthorized)}
            Some(cookie) => {UserID(cookie.value().to_string())}
        };

        request::Outcome::Success(user_id_hashed)

    }
}


/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
struct GoogleUserInfo {
    name: String,
    picture: String,
    id: String,
}

#[get("/login/google")]
fn google_login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["profile"]).unwrap()
}

#[get("/auth/google")]
async fn google_callback(
    db: Connection<Db>,
    token: TokenResponse<GoogleUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    // Use the token to retrieve the user's Google account information.
    let user_info_resp: Response = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://www.googleapis.com/oauth2/v1/userinfo")
        .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
        .send()
        .await
        .context("failed to complete request")?;
    
    // \"id\": \"106788192500279318769\",
    // \"name\": \"Paul Fornage\",
    // \"given_name\": \"Paul\",
    // \"family_name\": \"Fornage\",
    // \"picture\": \"https://lh3.googleusercontent.com/a/ACg8ocJt_pxHv-RtMGSxDzK_5-WcKdWt6cQiMk_chxQw9c_vfeAL93sz=s96-c\"



    let user_info: GoogleUserInfo = user_info_resp
        .json()
        .await
        .context("failed to deserialize response")?;
    let real_name = user_info.name;
    let image_url = user_info.picture;
    let user_id_from_google = user_info.id;

    let hashed_id = Sha3_256::digest(user_id_from_google);
    
    let str_hash = base16ct::lower::encode_string(&hashed_id);

    cookies.add_private(
        Cookie::build(("user_id_hashed", str_hash.clone()))
            .same_site(SameSite::Lax)
            .build(),
    );
    
    match User::find_by_id(str_hash, db).await {
        Ok(user) => {
            println!("user info found for {}", user.name);
        },
        Err(_) => {
            println!("user info not found, creating new one.");
            
        },
    }
    
    
    Ok(Redirect::to("/"))
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("user_id_hashed"));
    Redirect::to("/")
}

#[get("/")]
async fn index(db: Connection<Db>, user_id: UserID) -> String {
    match User::find_by_id(user_id.0, db).await {
        Ok(user) => {format!("{:?}", user)}
        Err(err) => {
            format!("error while finding user: {}", err)
        }
    }
    
}

#[get("/welcome")]
async fn welcome(db: Connection<Db>, user_id: UserID) -> String {
    match User::find_by_id(user_id.0, db).await {
        Ok(user) => {format!("{:?}", user)}
        Err(_) => { "User does not exist".to_string() }
    }
}

#[get("/", rank = 2)]
fn index_anonymous() -> &'static str {
    "Please login (/login/google)"
}

#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    rocket::build()
        .mount("/", routes![
            index, 
            google_callback, 
            google_login,
            logout,
            index_anonymous,
            welcome,
        ])
        .attach(make_cors())
        .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
        .attach(Db::init())
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}


#[cfg(test)]
mod comb_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[self::test]
    fn run_sample_game_generation() {
        let fr_word_list = WordList::try_from_file("fr_word_list.txt").unwrap();
        let comb = Comb::new_random(&fr_word_list);
        println!("Comb: {:#?}", comb);
        let game = Game::new(comb, fr_word_list);
        println!("Valid words: {:#?}", game.valid_words);

        println!("Game has {} possible words worth a total of {} points.", game.get_total_possible_words(), game.get_possible_points());
    }
}