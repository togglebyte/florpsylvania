use legion::{Resources, Schedule, World};
use std::collections::HashMap;

use tinybit::events::events;
use tinybit::{Renderer, StdoutTarget};

mod account;
mod inventory;
mod mainmenu;
mod player;
mod state;
mod stats;
mod ui;
mod world;

use mainmenu::MainMenu;
use state::{State, StateStack, Transition};

// mod pretendserver;

pub type Rend = Renderer<StdoutTarget>;
pub type NextState = Option<Transition>;

fn make_resources() -> Resources {
    let mut resources = Resources::default();

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let renderer = Renderer::new(stdout_renderer);
    resources.insert(renderer);

    resources.insert::<NextState>(None);

    resources
}

// -----------------------------------------------------------------------------
//     - Schedules -
// -----------------------------------------------------------------------------
struct Schedules {
    schedules: HashMap<State, Schedule>,
}

impl Schedules {
    fn new(resources: &mut Resources) -> Self {
        let mut schedules = HashMap::<State, Schedule>::new();
        schedules.insert(
            State::MainMenu(MainMenu),
            MainMenu::schedule(resources),
        );
        Self { schedules }
    }

    fn ensure_exists(&mut self, state: State, world: &mut World, resources: &mut Resources) {
        if !self.schedules.contains_key(&state) {
            let sched = state.schedule(world, resources);
            self.schedules.insert(state, sched);
        }
    }

    fn exec(&mut self, state: State, world: &mut World, resources: &mut Resources) {
        match self.schedules.get_mut(&state) {
            Some(systems) => systems.execute(world, resources),
            None => panic!("System not registered"),
        }
    }
}

// -----------------------------------------------------------------------------
//     - Run -
// -----------------------------------------------------------------------------
pub fn run() {
    // Start pretend server
    // pretendserver::serve();

    let mut resources = make_resources();
    let mut world = World::default();
    let mut schedules = Schedules::new(&mut resources);
    let mut state_stack = StateStack::new();

    for event in events(20) {
        resources.insert(event);
        let state = state_stack.top();
        schedules.exec(state, &mut world, &mut resources);
        let transition = resources.get::<NextState>().map(|t| *t).flatten();
        let transition = match transition {
            Some(t) => t,
            None => continue,
        };

        match transition {
            Transition::Quit => break,
            Transition::Pop => state_stack.pop(),
            Transition::Swap(new_state) => {
                state_stack.pop();
                state_stack.push(State::from(new_state));
                schedules.ensure_exists(new_state, &mut world, &mut resources);
            }
            Transition::Push(new_state) => {
                state_stack.push(State::from(new_state));
                schedules.ensure_exists(new_state, &mut world, &mut resources);
            }
        }

        resources.insert::<NextState>(None);
        resources.get_mut::<Rend>().map(|mut r| r.clear());
    }
}
