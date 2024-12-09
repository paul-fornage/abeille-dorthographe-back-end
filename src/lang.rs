
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct LanguageCode{
    pub code: String,
    pub name: String,
    pub letter_set: String,
    pub flag_emoji: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LangList(pub Vec<LanguageCode>);
impl LangList{
    pub fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
