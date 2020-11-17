use legion::systems::Builder;
use legion::{system, IntoQuery, Resources, World};
use tinybit::{Camera, WorldPos};

use crate::input::Input;
use crate::message::Message;
// use crate::net::Tx;

pub fn show_hide_cursor(world: &World, resources: &Resources) {
    let player_pos = <(&Player, &WorldPos)>::query()
        .iter(world)
        .map(|(_, p)| p)
        .next()
        .expect("No player?");

    resources.get_mut::<Cursor>().map(|mut cur| {
        cur.visible = !cur.visible;
        cur.pos = *player_pos;
    });
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Cursor {
    pub left: char,
    pub right: char,
    pub pos: WorldPos,
    pub visible: bool,
}

// -----------------------------------------------------------------------------
//     - Components -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Player(pub u8);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------

#[system(for_each)]
fn move_player(
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
        pos.x -= 1.0;
    }
    if input.contains(Input::RIGHT) {
        pos.x += 1.0;
    }
    if input.contains(Input::UP) {
        pos.y -= 1.0;
    }
    if input.contains(Input::DOWN) {
        pos.y += 1.0;
    }

    input.clear();

    // Send player position to the server
    // let _ = tx.send(Message::PlayerPos(*pos));
}

#[system(for_each)]
fn track_player(#[resource] camera: &mut Camera, player: &Player, pos: &mut WorldPos) {
    camera.track(*pos);
}

#[system]
fn move_cursor(
    #[resource] camera: &mut Camera,
    #[resource] cursor: &mut Cursor,
    #[resource] input: &mut Input,
) {
    if !cursor.visible {
        return;
    }

    if input.is_empty() {
        return;
    }

    if input.contains(Input::LEFT) {
        cursor.pos.x -= 1.0;
    }
    if input.contains(Input::RIGHT) {
        cursor.pos.x += 1.0;
    }
    if input.contains(Input::UP) {
        cursor.pos.y -= 1.0;
    }
    if input.contains(Input::DOWN) {
        cursor.pos.y += 1.0;
    }

    input.clear();
}

pub fn add_player_systems(builder: &mut Builder) {
    builder
        .add_system(move_player_system())
        .add_system(track_player_system())
        .add_system(move_cursor_system());
}
