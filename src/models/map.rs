use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapContentType {
    Monster,
    Resource,
    Workshop,
    Bank,
    GrandExchange,
    TasksMaster,
    Npc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapContent {
    pub content_type: MapContentType,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
    #[serde(flatten)]
    pub content: Option<MapContent>,
}
