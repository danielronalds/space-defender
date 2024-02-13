use sdl2::rect::{Rect, Point};

use crate::{Sprite, SPRITE_WIDTH, SPRITE_HEIGHT};

#[derive(Debug, Clone, Copy)]
enum EnemySprite {
    Stationary,
    Moving,
}

impl Sprite for EnemySprite {
    fn get_src_rect(&self) -> Rect {
        use EnemySprite::*;
        let (x, y) = match self {
            Stationary => (48, 0),
            Moving => (64, 0),
        };

        Rect::new(x, y, SPRITE_WIDTH, SPRITE_HEIGHT)
    }
}

/// A struct represnting the player's ship
pub struct Enemy {
    position: Point,
}

impl Enemy {
    pub fn new(position: Point) -> Self { Self { position } }

    pub fn position(&self) -> Point {
        self.position
    }
}

impl Sprite for Enemy {
    fn get_src_rect(&self) -> Rect {
        EnemySprite::Stationary.get_src_rect()
    }
}
