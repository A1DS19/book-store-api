use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateAuthor {
    pub first_name: String,
    pub last_name: String,
    pub biography: String,
}
