use legion::{system, Resources, Schedule, World};
use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::{term_size, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use crate::state::Transition;
use crate::state::State;
use crate::{NextState, Rend};
use crate::ui::TextField;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct SignIn;

impl SignIn {
    pub fn schedule(resources: &mut Resources) -> Schedule {
        let (width, height) = term_size().expect("Failed to get term size");

        let viewport_size = ScreenSize::new(width - 4, height - 4);
        let viewport = Viewport::new(ScreenPos::new(2, 2), viewport_size);

        resources.insert(SignInViewport(viewport));
        resources.insert(UsernameInput(TextField::new(None)));
        let mut password = TextField::new(None);
        password.password = true;
        resources.insert(PasswordInput(password));

        let mut schedule = Schedule::builder();
        schedule.add_system(render_system());
        schedule.add_system(input_fields_system());
        schedule.build()
    }
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
struct SignInViewport(Viewport);

struct UsernameInput(TextField);
struct PasswordInput(TextField);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------
#[system]
fn input_fields(
    #[resource] event: &mut Event,
    #[resource] username: &mut UsernameInput,
    #[resource] password: &mut PasswordInput,
    #[resource] next_state: &mut NextState,
) {

    let key_ev = match event {
        Event::Key(k) => k,
        _ => return,
    };

    match key_ev {
        KeyEvent {code: KeyCode::Tab, .. } => {
            menu_selection.next();
        }
    //     KeyEvent {code: KeyCode::Down, .. } => {
    //         menu_selection.prev();
    //     }
        KeyEvent {code: KeyCode::Enter, .. } => {
            *next_state = Some(Transition::Pop);
        }
        _ => return,
    }
}

#[system]
fn render(
    #[resource] viewport: &mut SignInViewport,
    #[resource] renderer: &mut Rend,
) {
    renderer.render(&mut viewport.0);
}
