use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct Character {
    pub id: u32,
    pub nickname: String,
    pub name: String,
}
