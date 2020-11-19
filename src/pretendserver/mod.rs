use std::thread;
use std::time::Duration;
use std::sync::atomic::AtomicUsize;

use legion::{Resources, Schedule, World};

mod combat;

use combat::add_combat_systems;

// This unit pool will provide a unique (not really) id
// for each unit, making it possible for clients to identify units
pub const UNIT_POOL: AtomicUsize = AtomicUsize::new(0);

pub const FPS: u64 = 20;

fn systems() -> Schedule {
    let mut builder = Schedule::builder();
    add_combat_systems(&mut builder);
    builder.build()
}

pub fn serve() {
    thread::spawn(|| {
        let mut sys = systems();
        let mut world = World::default();
        let mut resources = Resources::default();

        // Combat latency
        resources.insert(combat::MockLatency::new());

        loop {
            sys.execute(&mut world, &mut resources);
            thread::sleep(Duration::from_millis(1/FPS));
        }
    });
}
