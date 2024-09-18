use super::Response;

#[get("/")]
pub async fn index() -> Response<String> {
    todo!()
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
