use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    dto::authors::res_author_list::ResAuthorList, entities::author, guards::AuthenticatedUser,
};

use super::{ErrorResponse, Response, SuccessResponse};

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResAuthorList>, &'static str> {
    let db = db.inner();

    let authors = match author::Entity::find().all(db).await {
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
