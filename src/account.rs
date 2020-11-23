use legion::{system, Resources, Schedule};
use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::widgets::Border;
use tinybit::{term_size, ScreenPos, ScreenSize, Viewport};

use crate::state::Transition;
use crate::ui::TextField;
use crate::{NextState, Rend};

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
        schedule.add_system(draw_nonsense_system());
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
        KeyEvent {
            code: KeyCode::Tab, ..
        } => {
            // Choose next input
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            *next_state = Some(Transition::Pop);
        }
        _ => {}
    }

    if username.0.focus {
        username.0.event(*event);
    } else if password.0.focus {
        password.0.event(*event);
    }
}

#[system]
fn draw_nonsense(
    #[resource] viewport: &mut SignInViewport,
    #[resource] username: &mut UsernameInput,
    #[resource] password: &mut PasswordInput,
) {
    viewport.0.draw_widget(
        &Border::new("╔═╗║╝═╚║".to_string(), None, None),
        ScreenPos::zero(),
    );

    let x = viewport.0.size.width / 2 - 7;
    let y = viewport.0.size.height / 2 - 1;
    viewport.0.draw_widget(&username.0, ScreenPos::new(x, y));

    let x = viewport.0.size.width / 2 - 7;
    let y = viewport.0.size.height / 2 + 1;
    viewport.0.draw_widget(&password.0, ScreenPos::new(x, y));
}

#[system]
fn render(#[resource] viewport: &mut SignInViewport, #[resource] renderer: &mut Rend) {
    renderer.render(&mut viewport.0);
}
