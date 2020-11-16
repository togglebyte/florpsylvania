use legion::{Resources, Schedule, World};
use std::sync::mpsc;

use tinybit::events::{events, Event, KeyCode, KeyEvent};
use tinybit::{
    term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, WorldPos, WorldSize,
};

mod input;
mod message;
// mod net;
mod player;
mod rendering;
mod stats;
mod tilemap;

use input::Input;

pub type Rend = Renderer<StdoutTarget>;

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
    let camera_pos = WorldPos::new(20_000, 20_000);
    let mut camera = Camera::from_viewport(camera_pos, &viewport);
    camera.set_limit(1, 1, 1, 1);

    resources.insert(rendering::MainViewport(viewport));
    resources.insert(camera);

    // Rx / Tx
    // let (tx, rx): (net::Tx, net::Rx) = mpsc::channel();
    // resources.insert(tx);
    // resources.insert(rx);

    let mut tile = tilemap::TilemapMeh::new(tilemap::ThrowAwayThisProvider);
    tile.update(camera_pos, viewport_size);
    resources.insert(tile);

    resources
}

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------
fn systems() -> Schedule {
    Schedule::builder()
        .add_system(player::move_player_system())
        .add_system(player::track_player_system())
        // .add_system(net::net_recv_system())
        
        // Adding pixels to the buffers
        .add_system(stats::show_stats_system())
        .add_system(rendering::world_to_screen_system())
        .add_system(rendering::draw_tilemap_system())
        .add_system(rendering::draw_pixels_system())
        .add_system(rendering::draw_border_system())

        // Rendering should be the last system
        .add_system(rendering::render_system())
        .build()
}

pub fn run() {
    // World
    let mut world = World::default();

    world.push((
        player::Player(0),
        rendering::Glyph('@'),
        WorldPos::new(20_000, 20_000),
        ScreenPos::new(0, 0),
        stats::Hp(19),
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
