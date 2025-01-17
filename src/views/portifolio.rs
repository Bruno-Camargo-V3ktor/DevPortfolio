use rocket::{Route, get, routes};
use rocket_dyn_templates::{Template, context};

// ENDPOINTS
#[get("/", format = "text/html")]
pub fn main_page() -> Template {
    Template::render("index", context! { title: "Home Page" })
}

// FUNCTIONS
pub fn pages() -> Vec<Route> {
    routes![main_page]
}
