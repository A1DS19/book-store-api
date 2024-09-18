use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::{config::app_config::AppConfig, routes::auth::Claims};

pub struct AuthenticatedUser {
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("Token") {
            let config = match req.rocket().state::<AppConfig>() {
                Some(config) => config,
                None => {
                    return Outcome::Error((Status::InternalServerError, "AppConfig not found"));
                }
            };

            let claims = match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(config.jwt_secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(token_data) => token_data.claims,
                Err(_) => {
                    return Outcome::Error((Status::Unauthorized, "Invalid token"));
                }
            };

            return Outcome::Success(AuthenticatedUser { id: claims.sub });
        } else {
            return Outcome::Error((Status::Unauthorized, "Token not found"));
        }
    }
}
