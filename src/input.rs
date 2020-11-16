use tinybit::Vec2D;
use bitflags::bitflags;

bitflags! {
    pub struct Input: u8 {
        const LEFT = 1;
        const RIGHT = 2;
        const UP = 4;
        const DOWN = 8;
    }
}

impl Input {
    pub fn clear(&mut self) {
        self.bits = 0;
    }

    pub fn zero() -> Self {
        Self { bits: 0 }
    }
}
