use std::{collections::HashMap, sync::Arc};

use serde::{Serialize, Deserialize};
use serenity::{model::{guild::{Member, Role}, id::GuildId}, prelude::TypeMapKey};

use tokio::sync::RwLock;

pub(crate) type BotData = HashMap<GuildId, ServerData>;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GameData;

impl TypeMapKey for GameData {
    type Value = Arc<RwLock<BotData>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ServerData {
    pub(crate) games: HashMap<String, Game>,
}

impl ServerData {
    pub(crate) fn new_game(&mut self, name: String, owner: Member, role: Role) {
        self.games.insert(name.clone(), Game::new(name, owner, role));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Game {
    pub(crate) name: String,
    pub(crate) owner: Member,
    pub(crate) role: Role,
    pub(crate) admins: Vec<Member>,
    pub(crate) players: Vec<Member>,
}

impl Game {
    pub(crate) fn new(name: String, owner: Member, role: Role) -> Game {
        Game { name, owner, role, admins: Vec::new(), players: Vec::new() }
    }
}
