use legion::{system, Resources, Schedule, World};
use tinybit::events::Event;
use tinybit::{term_size, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use crate::state::Transition;
use crate::Rend;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct SignIn;

impl SignIn {
    pub fn schedule(resources: &mut Resources) -> Schedule {
        let (width, height) = term_size().expect("Failed to get term size");

        let viewport_size = ScreenSize::new(width - 4, height - 4);
        let viewport = Viewport::new(ScreenPos::new(2, 2), viewport_size);
        resources.insert(SignInViewport(viewport));

        let mut schedule = Schedule::builder();
        schedule.add_system(render_system());
        schedule.build()
    }
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
struct SignInViewport(Viewport);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------

#[system]
fn render(
    #[resource] viewport: &mut SignInViewport,
    #[resource] renderer: &mut Rend,
) {
    renderer.render(&mut viewport.0);
}
