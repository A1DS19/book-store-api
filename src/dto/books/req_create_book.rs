use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateBook {
    pub title: String,
    pub cover: String,
    pub year: i32,
    pub author_id: i32,
}
