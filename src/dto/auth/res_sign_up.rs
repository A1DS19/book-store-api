use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignUp {
    pub token: String,
}
