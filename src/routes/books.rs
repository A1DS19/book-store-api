use crate::entities::book;
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{dto::books::res_book_list::ResBookList, guards::AuthenticatedUser};

use super::{ErrorResponse, Response, SuccessResponse};

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResBookList>, &'static str> {
    let db = db.inner();

    let books = match book::Entity::find().all(db).await {
        Ok(books) => books,
        Err(_) => {
            return Err(ErrorResponse::new(
                Status::InternalServerError,
                "Failed to fetch books",
            ))
        }
    };

    Ok(SuccessResponse::new(
        Status::Ok,
        Json(ResBookList {
            books: books.clone(),
            total: books.len().clone(),
        }),
    ))
}

#[post("/")]
pub async fn create() -> Response<String> {
    todo!()
}

#[get("/<id>")]
pub async fn show(id: i32) -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update(id: i32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: i32) -> Response<String> {
    todo!()
}
