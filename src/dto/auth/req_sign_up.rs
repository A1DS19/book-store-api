use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
