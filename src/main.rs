use rocket::{self, launch};

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", rocket::routes![])
}
