use serde::{Deserialize, Serialize};
use tinybit::{Pixel, WorldPos};

#[derive(Serialize, Deserialize)]
pub enum Message {
    PlayerPos(WorldPos),
    MapRequest(WorldPos),
    Map(Vec<Pixel>),
}
