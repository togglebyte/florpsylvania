use legion::{Resources, Schedule, World};
use tinybit::events::Event;
use crate::state::Transition;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct SignIn;

impl SignIn {
    pub fn schedule(resources: &mut Resources) -> Schedule {
        let mut schedule = Schedule::builder();
        schedule.build()
    }

    // pub fn exec(&mut self, world: &mut World, resources: &mut Resources, event: Event) -> Transition {
    //     Transition::Empty
    // }
}
