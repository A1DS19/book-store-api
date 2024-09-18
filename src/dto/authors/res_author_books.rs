use crate::entities::book::Model as Book;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthorBooks {
    pub total: usize,
    pub books: Vec<Book>,
}
