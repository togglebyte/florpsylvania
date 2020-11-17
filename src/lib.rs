use legion::{IntoQuery, Read, Resources, Schedule, World, Entity};
use std::sync::mpsc;

use tinybit::events::{events, Event, KeyCode, KeyEvent};
use tinybit::{
    term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, WorldPos, WorldSize,
};

mod input;
mod message;
mod combat;
mod npc;
mod player;
mod rendering;
mod stats;
mod tilemap;
mod unit;

use input::Input;
use unit::Unit;
use player::{Cursor, Player, add_player_systems, show_hide_cursor};
use rendering::add_rendering_systems;
use stats::add_stats_systems;
use combat::add_combat_systems;

pub type Rend = Renderer<StdoutTarget>;

macro_rules! some_or_ret {
    ($ex:expr) => {
        match $ex {
            Some(val) => val,
            None => return,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
fn make_resources() -> Resources {
    let mut resources = Resources::default();

    // Input
    resources.insert(Input::zero());

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let renderer = Renderer::new(stdout_renderer);
    resources.insert(renderer);

    let (width, height) = term_size().expect("can't get the term size? can't play the game!");

    // UI viewport
    let viewport_size = ScreenSize::new(width, 4);
    let viewport = Viewport::new(ScreenPos::new(0, 0), viewport_size);
    resources.insert(stats::StatsViewport(viewport));

    // Main viewport
    let viewport_size = ScreenSize::new(80, 20);
    let viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);

    // Camera
    let camera_pos = WorldPos::zero();
    let mut camera = Camera::from_viewport(camera_pos, &viewport);
    camera.set_limit(4, 4, 4, 4);

    resources.insert(rendering::MainViewport(viewport));
    resources.insert(camera);

    // Rx / Tx
    // let (tx, rx): (net::Tx, net::Rx) = mpsc::channel();
    // resources.insert(tx);
    // resources.insert(rx);

    // Tilemap
    let mut tile = tilemap::TilemapMeh::new(tilemap::ThrowAwayThisProvider);
    tile.update(camera_pos, viewport_size);
    resources.insert(tile);

    // Cursor
    resources.insert(Cursor {
        left: '(',
        right: ')',
        visible: false,
        pos: WorldPos::zero(),
    });

    // Combat latency
    resources.insert(combat::MockLatency::new());

    resources
}

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------
fn systems() -> Schedule {
    let mut builder = Schedule::builder();
    add_player_systems(&mut builder);
    add_stats_systems(&mut builder);
    add_combat_systems(&mut builder);

    // Rendering last
    add_rendering_systems(&mut builder);
    builder.build()
}

pub fn run() {
    // World
    let mut world = World::default();

    world.push((
        Player(0),
        Unit,
        rendering::Glyph('@'),
        WorldPos::zero(),
        ScreenPos::new(0, 0),
        stats::Hp(19),
        combat::Weapon {
            damage: 3,
            range: 5,
            name: "Gun".to_string(),
        }
    ));

    world.push((
        npc::Npc,
        Unit,
        rendering::Glyph('E'),
        WorldPos::new(10.0, 2.0),
        ScreenPos::new(0, 0),
        stats::Hp(19),
        combat::Weapon {
            damage: 1,
            range: 4,
            name: "Another gun".to_string(),
        }
    ));

    // Resources
    let mut resources = make_resources();

    // Schedule
    let mut sched = systems();

    // Player
    // let mut player = ('@', camera_pos);

    for event in events(20) {
        match event {
            Event::Tick => {
                sched.execute(&mut world, &mut resources);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => match kc {
                KeyCode::Left => {
                    resources
                        .get_mut::<Input>()
                        .map(|mut inpt| inpt.insert(Input::LEFT));
                }
                KeyCode::Right => {
                    resources
                        .get_mut::<Input>()
                        .map(|mut inpt| inpt.insert(Input::RIGHT));
                }
                KeyCode::Up => {
                    resources
                        .get_mut::<Input>()
                        .map(|mut inpt| inpt.insert(Input::UP));
                }
                KeyCode::Down => {
                    resources
                        .get_mut::<Input>()
                        .map(|mut inpt| inpt.insert(Input::DOWN));
                }
                KeyCode::Char('s') => {
                    show_hide_cursor(&world, &resources);
                }
                KeyCode::Char('a') => {
                    resources.get_mut::<player::Cursor>().map(|mut cur| {
                        if cur.visible {
                            // Find target under cursor
                            let (player_ent, target_ent) = {
                                let target = <(&WorldPos, &Unit, Entity)>::query()
                                    .iter(&world)
                                    .map(|(_, _, e)| e)
                                    .next();

                                let target = some_or_ret!(target);

                                // Find the player entity
                                let player_ent = <(&Player, Entity)>::query()
                                    .iter(&world)
                                    .next()
                                    .map(|(_, e)| e);

                                let player_ent = some_or_ret!(player_ent);

                                (*player_ent, *target)
                            };

                            cur.visible = false;
                            combat::attack_target(&mut world, player_ent, target_ent);
                        }
                    });
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                resources.get_mut::<stats::StatsViewport>().map(|mut vp| {
                    let height = vp.0.size.height;
                    vp.0.resize(w, height)
                });

                resources.get_mut::<Rend>().map(|mut r| r.clear());
            }
        }
    }
}
