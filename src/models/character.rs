use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct Character {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub nickname: String,
    pub name: String,
}
