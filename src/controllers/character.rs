use axum::{ Json, response::IntoResponse };
use rusqlite::Connection;

use crate::models::character::Character;

const PATH_TO_DB: &str = "./db/mbtl.db";

pub async fn get_characters() -> impl IntoResponse {
    let conn = Connection::open(PATH_TO_DB).unwrap();
    let mut query = conn.prepare("SELECT name, nickname FROM character;").unwrap();
    let character_iter = query.query_map([], |row| {
        Ok(Character{
            id: None,
            name: row.get(0)?,
            nickname: row.get(1)?,
        })
    }).unwrap();

    let mut characters: Vec<Character> = Vec::new();
    for c in character_iter {
        characters.push(c.unwrap());
    }

    return Json(characters);
}
