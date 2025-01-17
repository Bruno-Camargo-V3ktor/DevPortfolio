mod models;
mod views;

use rocket::{self, launch};
use rocket_dyn_templates::Template;

#[launch]
fn launch() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", views::portifolio::pages())
}
