use legion::{Resources, Schedule, World};

use tinybit::events::Event;

use crate::state::{State, Transition};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct GameState;

impl From<GameState> for State {
    fn from(s: GameState) -> State {
        State::Game(s)
    }
}


impl GameState {
    pub fn schedule(resources: &mut Resources) -> Schedule {
        let mut schedule = Schedule::builder();
        schedule.build()
    }

    // pub fn exec(&mut self, world: &mut World, resources: &mut Resources, event: Event) -> Transition {
    //     panic!("running the game world");
    //     Transition::Empty
    // }
}
