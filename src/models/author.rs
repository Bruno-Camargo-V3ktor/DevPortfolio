use rocket::serde::{Deserialize, Serialize};

use super::miniature::Miniature;

// Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub nickname: String,
    pub full_name: String,
    pub description: String,

    pub avatar_image: String,

    pub icons: Vec<Icon>,
    pub contacts: Vec<Contact>,

    pub projects: Vec<Miniature>,
    pub articles: Vec<Miniature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Icon {
    pub icon: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub image_bg: String,
    pub url: String,
}
