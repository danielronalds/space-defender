use sdl2::rect::{Rect, Point};

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

/// A struct represnting the player's ship
pub struct Player {
    position: Point,
    sprite: PlayerSprite,
    angle: f64,
    rotating_left: bool,
    rotating_right: bool,
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
        self.sprite.get_src_rect()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Point::new(0, 0),
            sprite: PlayerSprite::Stationary,
            angle: 0.0,
            rotating_left: false,
            rotating_right: false,
        }
    }
}
