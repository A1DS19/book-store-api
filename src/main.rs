#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello])
}
