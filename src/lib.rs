use std::collections::HashMap;
use legion::{Resources, Schedule, World};

use tinybit::events::{events, Event};
use tinybit::{Renderer, StdoutTarget};

mod player;
mod stats;
mod account;
mod inventory;
mod mainmenu;
mod state;
mod world;

use mainmenu::MainMenu;
use state::{State, Transition};
use world::GameState;

mod pretendserver;

pub type Rend = Renderer<StdoutTarget>;

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
        match schedules.get_mut(current_state) {
            Some(systems) => systems.execute(&mut world, &mut resources),
            None => {
                eprintln!("System note registered");
                break;
            }
        }

        // Transition to next state
        let maybe_state = resources.get_mut::<NextState>().map(|t| *t);

        if let Some(transition) = maybe_state.flatten() {
            resources.insert::<NextState>(None);

            match transition {
                Transition::Quit => break,
                Transition::Swap(new_state) => {
                    state_stack.pop();
                    state_stack.push(State::from(new_state));
                    if !schedules.contains_key(&new_state) {
                        let sched = new_state.schedule(&mut world, &mut resources);
                        schedules.insert(new_state, sched);
                    }
                }
                Transition::Pop => {
                    state_stack.pop();
                }
                Transition::Push(new_state) => {
                    state_stack.push(State::from(new_state));
                }
            }

            // Clear render
            resources.get_mut::<Rend>().map(|mut r| r.clear());
        }
    }
}
