use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serenity::model::{guild::Role, id::GuildId, user::User};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BotData {
    servers: HashMap<GuildId, ServerData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ServerData {
    games: HashMap<String, Game>,
}

impl ServerData {
    pub(crate) fn new_game(&mut self, name: String, owner: User, role: Role) {
        self.games.insert(name.clone(), Game::new(name, owner, role));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Game {
    name: String,
    owner: User,
    role: Role,
    admins: Vec<User>,
    players: Vec<User>,
}

impl Game {
    pub(crate) fn new(name: String, owner: User, role: Role) -> Game {
        Game { name, owner, role, admins: Vec::new(), players: Vec::new() }
    }
}
