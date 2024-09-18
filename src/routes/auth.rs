use std::time::SystemTime;

use crate::{
    config::app_config::AppConfig,
    dto::auth::{res_me::ResMe, res_sign_up::ResSignUp},
    entities::{prelude::*, user},
    guards::AuthenticatedUser,
};
use bcrypt::{hash, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};

use crate::dto::auth::{req_sign_in::ReqSignIn, req_sign_up::ReqSignUp, res_sign_in::ResSignIn};

use super::{ErrorResponse, Response, SuccessResponse};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: u64,
}

async fn find_user_by_email(
    email: &str,
    db: &DatabaseConnection,
) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

#[get("/me")]
pub async fn me(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Response<Json<ResMe>, &'static str> {
    let db = db.inner();

    let authenticated_user = match User::find()
        .filter(user::Column::Id.eq(user.id))
        .one(db)
        .await
        .map_err(|_| ErrorResponse::new(Status::InternalServerError, "Internal Server Error"))?
    {
        Some(user) => user,
        None => {
            return Err(ErrorResponse::new(Status::NotFound, "User not found"));
        }
    };

    Ok(SuccessResponse::new(
        Status::Ok,
        Json(ResMe {
            id: authenticated_user.id,
            email: authenticated_user.email.clone(),
        }),
    ))
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    config: &State<AppConfig>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<Json<ResSignIn>, &'static str> {
    let db = db as &DatabaseConnection;

    match find_user_by_email(&req_sign_in.email, db).await {
        Ok(Some(user)) => {
            let is_valid_password =
                bcrypt::verify(&req_sign_in.password, &user.password).map_err(|_| {
                    ErrorResponse::new(Status::InternalServerError, "Internal Server Error")
                })?;

            if !is_valid_password {
                return Err(ErrorResponse::new(Status::Unauthorized, "Invalid password"));
            }

            let claims = Claims {
                sub: user.id,
                role: "user".to_string(),
                // 24 hours from the moment of login
                exp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + 60 * 60 * 24,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            )
            .unwrap();

            return Ok(SuccessResponse::new(Status::Ok, Json(ResSignIn { token })));
        }
        Ok(None) => {
            return Err(ErrorResponse::new(Status::Unauthorized, "User not found"));
        }
        Err(_) => {
            return Err(ErrorResponse::new(
                Status::InternalServerError,
                "Internal Server Error",
            ));
        }
    }
}

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(
    db: &State<DatabaseConnection>,
    req_sign_up: Json<ReqSignUp>,
) -> Response<ResSignUp, &'static str> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    match find_user_by_email(&req_sign_up.email, db).await {
        Ok(Some(_user)) => {
            return Err(ErrorResponse::new(Status::Conflict, "Email already exists"))
        }

        Ok(None) => {
            let hashed_password = hash(&req_sign_up.password, DEFAULT_COST).map_err(|_| {
                ErrorResponse::new(Status::InternalServerError, "Failed to hash password")
            })?;

            let insert_res = User::insert(user::ActiveModel {
                email: Set(req_sign_up.email.clone()),
                password: Set(hashed_password),
                first_name: Set(req_sign_up.first_name.to_string()),
                last_name: Set(req_sign_up.last_name.to_string()),
                ..Default::default()
            })
            .exec(db)
            .await;

            match insert_res {
                Ok(_) => {
                    return Ok(SuccessResponse::new(
                        Status::Created,
                        ResSignUp {
                            token: "account created".to_string(),
                        },
                    ));
                }

                Err(_) => {
                    return Err(ErrorResponse::new(
                        Status::InternalServerError,
                        "Internal Server Error",
                    ))
                }
            }
        }

        Err(_) => {
            return Err(ErrorResponse::new(
                Status::InternalServerError,
                "Internal Server Error",
            ))
        }
    }
}
