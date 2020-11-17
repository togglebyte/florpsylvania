use legion::*;
use tinybit::{Camera, WorldPos};

use crate::input::Input;
use crate::message::Message;
// use crate::net::Tx;

#[derive(Debug)]
pub struct Player(pub u8);

#[derive(Debug)] 
pub struct Cursor {
    pub left: char,
    pub right: char,
    pub pos: WorldPos,
    pub visible: bool,
}

#[system(for_each)]
pub fn move_player(
    #[resource] input: &mut Input,
    // #[resource] tx: &mut Tx,
    #[resource] cursor: &Cursor,
    player: &Player,
    pos: &mut WorldPos,
) {
    if input.is_empty() {
        return;
    }

    if cursor.visible {
        return;
    }

    if input.contains(Input::LEFT) {
        pos.x -= 1;
    }
    if input.contains(Input::RIGHT) {
        pos.x += 1;
    }
    if input.contains(Input::UP) {
        pos.y -= 1;
    }
    if input.contains(Input::DOWN) {
        pos.y += 1;
    }

    input.clear();

    // Send player position to the server
    // let _ = tx.send(Message::PlayerPos(*pos));
}

#[system(for_each)]
pub fn track_player(#[resource] camera: &mut Camera, player: &Player, pos: &mut WorldPos) {
    camera.track(*pos);
}

#[system]
pub fn move_cursor(#[resource] camera: &mut Camera, #[resource] cursor: &mut Cursor, #[resource] input: &mut Input) {
    if !cursor.visible {
        return;
    }

    if input.is_empty() {
        return;
    }

    if input.contains(Input::LEFT) {
        cursor.pos.x -= 1;
    }
    if input.contains(Input::RIGHT) {
        cursor.pos.x += 1;
    }
    if input.contains(Input::UP) {
        cursor.pos.y -= 1;
    }
    if input.contains(Input::DOWN) {
        cursor.pos.y += 1;
    }

    input.clear();
}
