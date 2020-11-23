use legion::systems::Builder;
use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery, World};
use tinybit::WorldPos;

pub fn attack_target(world: &mut World, attacker: Entity, target: Entity, target_pos: WorldPos) {
    if let Some(mut entry) = world.entry(attacker) {
        entry.add_component(Target {
            ent: target,
            pos: target_pos,
        });

    }
}

// -----------------------------------------------------------------------------
//     - Components -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Target {
    pub ent: Entity,
    pub pos: WorldPos,
}

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
fn update_targets(world: &SubWorld, target: &mut Target) {
    <(&WorldPos, Entity)>::query()
        .iter(world)
        .for_each(|(pos, ent)| {
            if target.ent == *ent {
                target.pos = *pos;
                return;
            }
        });
}

#[system(for_each)]
fn combat(
    world: &SubWorld,
    #[resource] latency: &mut MockLatency,
    target: &Target,
    weapon: &Weapon,
    attacker_pos: &WorldPos,
    entity: &Entity,
) {
    eprintln!("{:?}", "this is running");

    let len = (target.pos.to_vector() - attacker_pos.to_vector()).length();
    eprintln!("len: {:?}", len);
    if len > weapon.range as f32 {
        return;
    }

    eprintln!("{:?}", "within range");

    // world.add_compon

    // Cool down
    // Weapon range
    // Target position
    // Attacker position
    //
    // 1. Unit attacks if within range
    //  1.2 apply damage
    //  1.3 if either party is dead, stop combat
    // 2. Initiate cooldown
    // thread::sleep_ms(latency.next());
}

#[system(for_each)]
fn cooldown(#[resource] latency: &mut MockLatency, combat: &Combat) {
    // Cool down
    // Once cooldown reaches zero, remove component
}

pub fn add_combat_systems(builder: &mut Builder) {
    builder
        .add_system(update_targets_system())
        .add_system(combat_system())
        .add_system(cooldown_system());
}
