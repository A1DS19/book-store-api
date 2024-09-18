use crate::{
    dto::{authors::req_create_author, books::req_create_book::ReqCreateBook},
    entities::{author, book},
};
use rocket::{futures::TryFutureExt, http::Status, serde::json::Json, Data, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

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

#[post("/", data = "<req_book_body>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    req_book_body: Json<ReqCreateBook>,
    user: AuthenticatedUser,
) -> Response<Json<i32>, &'static str> {
    let db = db.inner();

    let new_book = book::ActiveModel {
        title: Set(req_book_body.title.clone()),
        cover: Set(req_book_body.cover.clone()),
        year: Set(req_book_body.year.clone()),
        author_id: Set(req_book_body.author_id.clone()),
        user_id: Set(user.id),
        ..Default::default()
    };

    match book::Entity::insert(new_book).exec(db).await {
        Ok(created_book) => Ok(SuccessResponse::new(
            Status::Created,
            Json(created_book.last_insert_id),
        )),
        Err(_) => Err(ErrorResponse::new(
            Status::InternalServerError,
            "Failed to create book",
        )),
    }
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Response<Json<book::Model>, &'static str> {
    let db = db.inner();

    let book = book::Entity::find_by_id(id)
        .one(db)
        .map_err(|_| ErrorResponse::new(Status::InternalServerError, "Failed to fetch book"))
        .await?;

    match book {
        Some(book) => Ok(SuccessResponse::new(Status::Ok, Json(book))),
        None => Err(ErrorResponse::new(Status::NotFound, "Book not found")),
    }
}

#[put("/<id>", data = "<req_book_body>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    id: i32,
    req_book_body: Json<ReqCreateBook>,
) -> Response<Json<i32>, &'static str> {
    let db = db.inner();

    let book = book::Entity::find_by_id(id)
        .one(db)
        .map_err(|_| ErrorResponse::new(Status::BadRequest, "error"))
        .await?;

    let mut book: book::ActiveModel = book.unwrap().into();

    book.title = Set(req_book_body.title.clone());
    book.cover = Set(req_book_body.cover.clone());
    book.year = Set(req_book_body.year.clone());

    let book = book
        .update(db)
        .map_err(|_| ErrorResponse::new(Status::InternalServerError, "error"))
        .await?;

    Ok(SuccessResponse::new(Status::Ok, Json(book.id)))
}

#[delete("/<id>")]
pub async fn delete(id: i32) -> Response<String> {
    todo!()
}
