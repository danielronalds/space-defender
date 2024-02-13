use sdl2::rect::{Point, Rect};

use crate::{SPRITE_HEIGHT, SPRITE_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum PlayerSprite {
    Stationary,
    Moving,
}

impl PlayerSprite {
    pub fn get_src_rect(&self) -> Rect {
        use PlayerSprite::*;
        let (x, y) = match self {
            Stationary => (0, 0),
            Moving => (16, 0),
        };

        Rect::new(x, y, SPRITE_WIDTH, SPRITE_HEIGHT)
    }
}

pub const MAX_SPEED: u32 = 28;
pub const ACCELERATION: u32 = 2;
pub const DECCELERATION: u32 = 1;

/// A struct represnting the player's ship
pub struct Player {
    position: Point,
    angle: f64,
    rotating_left: bool,
    rotating_right: bool,
    thrusters: bool,
    speed: u32,
}

impl Player {
    pub fn position(&self) -> Point {
        self.position
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn set_rotating_left(&mut self, rotating_left: bool) {
        self.rotating_left = rotating_left;
    }

    pub fn set_rotating_right(&mut self, rotating_right: bool) {
        self.rotating_right = rotating_right;
    }

    pub fn rotating_right(&self) -> bool {
        self.rotating_right
    }

    pub fn rotating_left(&self) -> bool {
        self.rotating_left
    }

    pub fn get_src_rect(&self) -> Rect {
        match self.thrusters() {
            true => PlayerSprite::Moving.get_src_rect(),
            false => PlayerSprite::Stationary.get_src_rect(),
        }
    }

    pub fn thrusters(&self) -> bool {
        self.thrusters
    }

    pub fn set_thrusters(&mut self, thrusters: bool) {
        self.thrusters = thrusters;
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Point::new(0, 0),
            angle: 0.0,
            rotating_left: false,
            rotating_right: false,
            thrusters: false,
            speed: 0,
        }
    }
}
