use legion::{Resources, World, Schedule};
use tinybit::events::Event;

use crate::account::SignIn;
use crate::world::GameState;
use crate::mainmenu::MainMenu;
use crate::inventory::Inventory;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum State {
    MainMenu(MainMenu),
    SignIn(SignIn),
    // Inventory(Inventory),
    Game(GameState),
}

impl State {
    pub fn schedule(&self, world: &mut World, resources: &mut Resources) -> Schedule {
        match self {
            Self::MainMenu(_) => MainMenu::schedule(resources),
            Self::SignIn(_) => SignIn::schedule(resources),
            Self::Game(_) => GameState::schedule(world, resources),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum Transition {
    Pop,
    Push(State),
    Swap(State),
    Quit,
}

// impl State {
//     pub fn exec(&mut self, world: &mut World, resources: &mut Resources, event: Event) -> Transition {
//         match self {
//             State::MainMenu(state) => state.exec(world, resources, event),
//             State::SignIn(state) => state.exec(world, resources, event),
//             State::Inventory(state) => state.exec(world, resources, event),
//             State::Game(state) => state.exec(world, resources, event),
//         }
//     }
// }
