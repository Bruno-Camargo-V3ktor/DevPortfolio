use rocket::serde::{Deserialize, Serialize};

// Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Miniature {
    pub id: String,
    pub image: String,
    pub title: String,
}
