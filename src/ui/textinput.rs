use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::widgets::Widget;
use tinybit::{Color, Pixel, ScreenPos, ScreenSize};

pub struct TextField {
    pub text: String,
    pub password: bool,
    pub focus: bool,
    pub submit: bool,
    color: Option<Color>,
    cursor: usize,
}

impl TextField {
    pub fn new(color: Option<Color>) -> Self {
        Self {
            text: String::new(),
            password: false,
            focus: false,
            submit: false,
            color,
            cursor: 0,
        }
    }

    pub fn event(&mut self, event: Event) {
        if !self.focus {
            return;
        }

        let key_code = match event {
            Event::Key(KeyEvent { code: k, .. }) => k,
            _ => return,
        };

        match key_code {
            KeyCode::Left if self.cursor > 0 => {
                self.cursor -= 1;
            }
            KeyCode::Right if self.cursor < self.text.len() => {
                self.cursor += 1;
            }
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.text.remove(self.cursor);
            }
            KeyCode::Backspace if self.text.len() > 0 => {
                self.text.remove(self.cursor);
                if self.cursor > self.text.len() {
                    self.cursor = self.text.len();
                }
            }
            KeyCode::Enter => {
                self.submit = true;
            }
            KeyCode::Char(c) => {
                self.text.insert(self.cursor, c);
                self.cursor += 1;
            }
            _ => {}
        }
    }
}

impl Widget for TextField {
    fn pixels(&self, _size: ScreenSize) -> Vec<Pixel> {
        self.text
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if self.password {
                    (x as u16, '*')
                } else {
                    (x as u16, c)
                }
            })
            .map(|(x, c)| Pixel::new(c, ScreenPos::new(x, 0), self.color))
            .collect()
    }
}
