use raylib::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Brick {
    pub position: Vector2,
    pub height: f32,
    pub width: f32,
    pub is_visible: bool,
}

pub trait BreakOnCollision {
    fn destory(&mut self) {
        drop(self);
    }

    fn break_wall(&mut self);
}

pub trait ClubHorizontal {
    fn club(&mut self);
}

impl BreakOnCollision for Brick {
    fn break_wall(&mut self) {
        if self.is_visible {
            self.is_visible = !self.is_visible;
        }
    }

    fn destory(&mut self) {
        self.is_visible = !self.is_visible
    }
}
