pub mod auth;
pub mod authors;
pub mod books;

use rocket::http::Status;
use rocket::response::Responder;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse<T>(pub (Status, T));

pub type Response<T, X = String> = Result<SuccessResponse<T>, ErrorResponse<X>>;

impl<T> SuccessResponse<T> {
    fn new(status_code: Status, data: T) -> Self {
        Self((status_code, data))
    }
}

impl<T> ErrorResponse<T> {
    fn new(status_code: Status, data: T) -> Self {
        Self((status_code, data))
    }
}
