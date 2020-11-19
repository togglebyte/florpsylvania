use serde::{Deserialize, Serialize};
use tinybit::{Pixel, WorldPos};

#[derive(Serialize, Deserialize)]
pub enum Message {
    PlayerLogin(String, String),
}
