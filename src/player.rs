use std::{borrow::Borrow, cell::RefCell};

use super::*;

thread_local! {
    pub static CONTROLS: RefCell<[(KeyCode, Vec2); 4]> = RefCell::new([
        (KeyCode::W, Vec2::NEG_Y),
        (KeyCode::A, Vec2::NEG_X),
        (KeyCode::S, Vec2::Y),
        (KeyCode::D, Vec2::X),
    ])
}

pub struct Player {
    pub pos: Vec2,
    /// The current velocity of the player
    pub vel: Vec2,
    /// How much the player can accelerate in a single frame
    pub acceleration: f32,
    pub max_vel: f32,
    pub damping: f32,
}
impl Player {
    pub fn update(&mut self, dt: f32) {
        let mut accel = Vec2::ZERO;
        CONTROLS.with_borrow(|keys| {
            for (key, dir) in keys {
                if macroquad::input::is_key_down(*key) {
                    accel += *dir;
                }
            }
        });
        self.vel += accel*self.acceleration*dt;

        if self.vel.length() > self.max_vel {
            self.vel = self.vel.normalize() * self.max_vel;
        }
        self.pos += self.vel;
        self.vel *= self.damping;
        if self.vel.x.abs() <= 0.01 {
            self.vel.x = 0.;
        }
        if self.vel.y.abs() <= 0.01 {
            self.vel.y = 0.;
        }
    }
}
pub fn new() -> Player {
    Player {
        pos: Vec2::splat(0.001), // Simple fix, because we use ceil for tile drawing
        vel: Vec2::ZERO,
        acceleration: 2.5,
        max_vel: 10.,
        damping: 0.9,
    }
}