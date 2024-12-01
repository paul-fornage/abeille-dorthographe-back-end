use rsntp::AsyncSntpClient;
use chrono::{DateTime, Datelike, Local, Utc};

async fn get_local_time() -> DateTime<Local> {
    let client = AsyncSntpClient::new();
    let result = client.synchronize("pool.ntp.org").await.unwrap();
// TODO, NTP error?!?!
    DateTime::from(result.datetime().into_chrono_datetime().unwrap())
}

pub async fn get_local_date() -> chrono::NaiveDate {
    get_local_time().await
        .naive_local()
        .date()
}

pub fn get_unique_letters(word: &str) -> Vec<char> {
    let mut present_chars: Vec<char> = Vec::with_capacity(word.len());
    for char in word.chars() {
        if !present_chars.contains(&char) {
            present_chars.push(char);
        }
    }
    present_chars
}

pub fn get_point_value(word: &str, is_panagram: bool) -> u64{
    
    let length: u64 = word.len() as u64 ;
    let points: u64 = length.saturating_sub(3);
    
    if is_panagram {
        points * 2u64
    } else {
        points
    }
}