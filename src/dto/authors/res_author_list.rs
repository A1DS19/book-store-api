use crate::entities::author::Model as Author;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthorList {
    pub total: usize,
    pub authors: Vec<Author>,
}
