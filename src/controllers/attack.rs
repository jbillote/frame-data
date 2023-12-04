use axum::{
    extract::Path,
    Json,
    response::IntoResponse
};
use rusqlite::Connection;

use crate::models::attack::Attack;

const PATH_TO_DB: &str = "./db/mbtl.db";

pub async fn get_attacks(Path(character_name): Path<String>) -> impl IntoResponse {
    let conn = Connection::open(PATH_TO_DB).unwrap();
    let mut query = conn.prepare("SELECT name, input FROM move WHERE charId = 
        (SELECT id FROM character WHERE LOWER(name) = LOWER(:name) OR 
        LOWER(nickname) = LOWER(:name));").unwrap();
    let move_iter = query.query_map(&[(":name", &character_name)], |row| {
        Ok(Attack{
            name: row.get(0)?,
            input: row.get(1)?,
            damage: None,
            block: None,
            cancel: None,
            property: None,
            cost: None,
            attribute: None,
            startup: None,
            active: None,
            recovery: None,
            overall: None,
            advantage: None,
            invuln: None,
        })
    }).unwrap();

    let mut moves: Vec<Attack> = Vec::new();
    for m in move_iter {
        moves.push(m.unwrap());
    }

    return Json(moves);
}

pub async fn get_attack(Path((character_name, input)): Path<(String, String)>) -> impl IntoResponse {
    let conn = Connection::open(PATH_TO_DB).unwrap();
    let mut query = conn.prepare("SELECT name, input, damage, block, cancel, 
        property, cost, attribute, startup, active, recovery, overall, 
        advantage, invuln FROM move WHERE charId = (SELECT id FROM character 
        WHERE LOWER(name) = LOWER(:name) OR LOWER(nickname) = LOWER(:name)) AND 
        LOWER(input) = LOWER(:input);").unwrap();
    let move_iter = query.query_map(&[(":name", &character_name), (":input", &input)], |row| {
        Ok(Attack{
            name: row.get(0)?,
            input: row.get(1)?,
            damage: row.get(2)?,
            block: row.get(3)?,
            cancel: row.get(4)?,
            property: row.get(5)?,
            cost: row.get(6)?,
            attribute: row.get(7)?,
            startup: row.get(8)?,
            active: row.get(9)?,
            recovery: row.get(10)?,
            overall: row.get(11)?,
            advantage: row.get(12)?,
            invuln: row.get(13)?,
        })
    }).unwrap();

    let mut attacks: Vec<Attack> = Vec::new();
    for m in move_iter {
        attacks.push(m.unwrap());
    }

    return Json(attacks.drain(0..1).next());
}
