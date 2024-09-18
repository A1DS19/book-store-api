use rocket::{futures::TryFutureExt, http::Status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    dto::authors::{req_create_author::ReqCreateAuthor, res_author_list::ResAuthorList},
    entities::author::{
        ActiveModel as AuthorActiveModel, Entity as AuthorEntity, Model as AuthorModel,
    },
    guards::AuthenticatedUser,
};

use super::{ErrorResponse, Response, SuccessResponse};

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResAuthorList>, &'static str> {
    let db = db.inner();

    let authors = match AuthorEntity::find().all(db).await {
        Ok(authors) => authors,
        Err(_) => {
            return Err(ErrorResponse::new(
                Status::InternalServerError,
                "Failed to fetch authors",
            ))
        }
    };

    Ok(SuccessResponse::new(
        Status::Ok,
        Json(ResAuthorList {
            authors: authors.clone(),
            total: authors.len().clone(),
        }),
    ))
}

#[post("/", data = "<req_author_body>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    req_author_body: Json<ReqCreateAuthor>,
    user: AuthenticatedUser,
) -> Response<Json<i32>, String> {
    let db = db.inner();

    let new_author = AuthorActiveModel {
        first_name: Set(req_author_body.first_name.clone()),
        last_name: Set(req_author_body.last_name.clone()),
        user_id: Set(user.id),
        biography: Set(req_author_body.biography.clone()),
        ..Default::default()
    };

    match AuthorEntity::insert(new_author).exec(db).await {
        Ok(author_res) => Ok(SuccessResponse::new(
            Status::Created,
            Json(author_res.last_insert_id),
        )),
        Err(e) => Err(ErrorResponse::new(
            Status::InternalServerError,
            format!("Failed to create author: {}", e),
        )),
    }
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Response<Json<AuthorModel>, &'static str> {
    let db = db.inner();

    let author = AuthorEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| ErrorResponse::new(Status::InternalServerError, "Failed to fetch author"))?;

    match author {
        Some(author) => Ok(SuccessResponse::new(Status::Ok, Json(author))),
        None => Err(ErrorResponse::new(Status::NotFound, "Book not found")),
    }
}

#[put("/<id>", data = "<req_author_body>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    id: i32,
    req_author_body: Json<ReqCreateAuthor>,
) -> Response<Json<i32>, &'static str> {
    let db = db.inner();

    let author = AuthorEntity::find_by_id(id)
        .one(db)
        .map_err(|_| ErrorResponse::new(Status::BadRequest, "error"))
        .await?;

    let mut author: AuthorActiveModel = author.unwrap().into();

    author.first_name = Set(req_author_body.first_name.clone());
    author.last_name = Set(req_author_body.last_name.clone());
    author.biography = Set(req_author_body.biography.clone());

    let author = author
        .update(db)
        .map_err(|_| ErrorResponse::new(Status::BadRequest, "error"))
        .await?;

    Ok(SuccessResponse::new(Status::Ok, Json(author.id)))
}

#[delete("/<id>")]
pub async fn delete(id: i32) -> Response<String> {
    todo!()
}
