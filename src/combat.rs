use std::thread;

use tinybit::WorldPos;
use legion::systems::Builder;
use legion::{system, Entity, World, IntoQuery};
use legion::world::SubWorld;

pub fn attack_target(world: &mut World, attacker: Entity, target: Entity) {
    if let Some(mut entry) = world.entry(attacker) {
        entry.add_component(Target(target));
    }
}

// -----------------------------------------------------------------------------
//     - Components -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Target(pub Entity);

#[derive(Debug)]
pub struct Attackable;

#[derive(Debug)]
pub struct Combat;

#[derive(Debug)]
pub struct Cooldown(pub usize);

#[derive(Debug)]
pub struct Weapon {
    pub damage: usize,
    pub range: u16,
    pub name: String,
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
pub struct MockLatency {
    index: usize,
    latencies: Vec<u32>,
}

impl MockLatency {
    pub fn new() -> Self {
        Self {
            index: 0,
            latencies: vec![10, 43, 100, 200, 126, 99, 64, 11, 13],
        }
    }

    pub fn next(&mut self) -> u32 {
        let lat = &self.latencies[self.index];
        self.index += 1;
        if self.index == self.latencies.len() {
            self.index = 0;
        }
        *lat
    }
}

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------
#[system(for_each)]
#[read_component(WorldPos)]
fn combat(
    world: &SubWorld,
    #[resource] latency: &mut MockLatency,
    target: &Target,
    weapon: &Weapon,
    attacker_pos: &WorldPos,
) {
    let target_pos = match <&WorldPos>::query().iter(world).next() {
        Some(p) => p,
        None => return
    };

    if (target_pos.to_vector() - attacker_pos.to_vector()).length() as u16 > weapon.range {
        return
    }

    // Cool down
    // Weapon range
    // Target position
    // Attacker position
    //
    // 1. Unit attacks if within range
    //  1.2 apply damage
    //  1.3 if either party is dead, stop combat
    // 2. Initiate cooldown
    thread::sleep_ms(latency.next());
}

#[system(for_each)]
fn cooldown(#[resource] latency: &mut MockLatency, combat: &Combat) {
    // Cool down
    // Once cooldown reaches zero, remove component
}

pub fn add_combat_systems(builder: &mut Builder) {
    builder
        .add_system(combat_system())
        .add_system(cooldown_system());
}
