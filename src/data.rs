use serde::{Serialize, Deserialize};

use serenity::model::user::User;


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerData {
    games: Vec<Game>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Game {
    name: String,
    owner: User,
    players: Vec<User>,
}
