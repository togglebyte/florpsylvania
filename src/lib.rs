use std::collections::HashMap;
// use legion::{Entity, IntoQuery, Read, Resources, Schedule, World};
use legion::{Resources, Schedule, World};

use tinybit::events::{events, Event};
use tinybit::{Renderer, StdoutTarget};
// use tinybit::events::{events, Event, KeyCode, KeyEvent};
// use tinybit::{
//     term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, WorldPos, WorldSize,
// };

// mod message;
// mod npc;
// mod player;
// mod rendering;
// mod stats;
// mod tilemap;
// mod unit;
// mod net;
mod account;
mod inventory;
mod mainmenu;
mod state;
mod world;

use mainmenu::MainMenu;
use state::{State, Transition};
use world::GameState;

mod pretendserver;

// use input::Input;
// use player::{add_player_systems, show_hide_cursor, Cursor, Player};
// use rendering::add_rendering_systems;
// use stats::add_stats_systems;
// use tilemap::TilemapMeh;
// use unit::Unit;

pub type Rend = Renderer<StdoutTarget>;

// macro_rules! some_or_ret {
//     ($ex:expr) => {
//         match $ex {
//             Some(val) => val,
//             None => return,
//         }
//     };
// }

// // -----------------------------------------------------------------------------
// //     - Resources -
// // -----------------------------------------------------------------------------
// fn make_resources(tx: net::Tx, rx: net::Rx) -> Resources {
//     let mut resources = Resources::default();

//     // Input
//     resources.insert(Input::zero());

//     // Renderer
//     let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
//     let renderer = Renderer::new(stdout_renderer);
//     resources.insert(renderer);

//     let (width, height) = term_size().expect("can't get the term size? can't play the game!");

//     // UI viewport
//     let viewport_size = ScreenSize::new(width, 4);
//     let viewport = Viewport::new(ScreenPos::new(0, 0), viewport_size);
//     resources.insert(stats::StatsViewport(viewport));

//     // Main viewport
//     let viewport_size = ScreenSize::new(80, 20);
//     let viewport = Viewport::new(ScreenPos::new(0, 7), viewport_size);

//     // Camera
//     let camera_pos = WorldPos::zero();
//     let mut camera = Camera::from_viewport(camera_pos, &viewport);
//     camera.set_limit(4, 4, 4, 4);

//     resources.insert(rendering::MainViewport(viewport));
//     resources.insert(camera);

//     // Tilemap
//     let mut tile = TilemapMeh::new(tilemap::ThrowAwayThisProvider);
//     tile.update(camera_pos, viewport_size);
//     resources.insert(tile);

//     // Cursor
//     resources.insert(Cursor {
//         left: '(',
//         right: ')',
//         visible: false,
//         pos: WorldPos::zero(),
//     });

//     resources.insert(net::FakeSocket::new());

//     resources
// }

// // -----------------------------------------------------------------------------
// //     - Systems -
// // -----------------------------------------------------------------------------
// fn systems() -> Schedule {
//     let mut builder = Schedule::builder();
//     add_player_systems(&mut builder);
//     add_stats_systems(&mut builder);

//     // Rendering last
//     add_rendering_systems(&mut builder);
//     builder.build()
// }

// pub fn run() {
//     // Start the pretend server.
//     // This should be removed once we have an
//     // actual server (obivously)
//     pretendserver::serve();

//     // World
//     let mut world = World::default();

//     // Resources
//     let mut resources = make_resources();

//     // Schedule
//     let mut sched = systems();

//     for event in events(20) {
//         match event {
//             Event::Tick => {
//                 sched.execute(&mut world, &mut resources);
//             }
//             Event::Key(KeyEvent {
//                 code: KeyCode::Esc, ..
//             }) => break,
//             Event::Key(KeyEvent { code: kc, .. }) => match kc {
//                 KeyCode::Left => {
//                     resources
//                         .get_mut::<Input>()
//                         .map(|mut inpt| inpt.insert(Input::LEFT));
//                 }
//                 KeyCode::Right => {
//                     resources
//                         .get_mut::<Input>()
//                         .map(|mut inpt| inpt.insert(Input::RIGHT));
//                 }
//                 KeyCode::Up => {
//                     resources
//                         .get_mut::<Input>()
//                         .map(|mut inpt| inpt.insert(Input::UP));
//                 }
//                 KeyCode::Down => {
//                     resources
//                         .get_mut::<Input>()
//                         .map(|mut inpt| inpt.insert(Input::DOWN));
//                 }
//                 KeyCode::Char('s') => {
//                     show_hide_cursor(&world, &resources);
//                 }
//                 KeyCode::Char('a') => {
//                     resources.get_mut::<player::Cursor>().map(|mut cur| {
//                         if cur.visible {
//                             // Find target under cursor
//                             let (player_ent, target_ent, target_pos) = {
//                                 let target = <(&WorldPos, &Unit, Entity)>::query()
//                                     .iter(&world)
//                                     .filter(|(pos, _, e)| **pos == cur.pos)
//                                     .map(|(pos, _, e)| (pos, e))
//                                     .next();

//                                 let (target_pos, target) = some_or_ret!(target);

//                                 // Find the player entity
//                                 let player_ent = <(&Player, Entity)>::query()
//                                     .iter(&world)
//                                     .next()
//                                     .map(|(_, e)| e);

//                                 let player_ent = some_or_ret!(player_ent);

//                                 (*player_ent, *target, *target_pos)
//                             };

//                             cur.visible = false;
//                             unimplemented!();
//                             // Send instruction to the server that we want to attack
//                             // entity with id X
//                             // combat::attack_target(&mut world, player_ent, target_ent, target_pos);
//                         }
//                     });
//                 }
//                 _ => {}
//             },
//             Event::Resize(w, h) => {
//                 resources.get_mut::<stats::StatsViewport>().map(|mut vp| {
//                     let height = vp.0.size.height;
//                     vp.0.resize(w, height)
//                 });

//                 resources.get_mut::<Rend>().map(|mut r| r.clear());
//             }
//         }
//     }
// }

pub type NextState = Option<Transition>;

pub fn run() {
    // Start pretend server
    pretendserver::serve();

    let mut resources = Resources::default();
    let mut world = World::default();
    let mut net_schedule = Schedule::builder().build();

    // Schedules
    let mut schedules = HashMap::<State, Schedule>::new();
    schedules.insert(State::MainMenu(MainMenu), MainMenu::schedule(&mut resources));

    let mut state_stack = vec![State::MainMenu(MainMenu)];

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let renderer = Renderer::new(stdout_renderer);
    resources.insert(renderer);

    // States 
    resources.insert::<NextState>(None);

    for event in events(20) {
        if let Event::Tick = event {
            net_schedule.execute(&mut world, &mut resources);
        }

        resources.insert(event);

        // Execute current state systems
        let current_state = state_stack.last().unwrap();
        schedules.get_mut(current_state).map(|systems| {
            systems.execute(&mut world, &mut resources);
        });


        // Transition to next state
        let maybe_state = resources.get_mut::<NextState>().map(|t| *t);

        if let Some(transition) = maybe_state.flatten() {
            resources.insert::<NextState>(None);

            match transition {
                Transition::Quit => break,
                Transition::Swap(new_state) => {
                    state_stack.pop();
                    state_stack.push(State::from(new_state));
                }
                Transition::Pop => {
                    state_stack.pop();
                }
                Transition::Push(new_state) => {
                    state_stack.push(State::from(new_state));
                }
            }
        }
    }
}
