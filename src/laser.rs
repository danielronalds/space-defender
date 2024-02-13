use sdl2::rect::{Point, Rect};

use crate::{Sprite, SPRITE_HEIGHT, SPRITE_WIDTH};

pub fn update_laser(laser: Laser) -> Option<Laser> {
    let mut laser = laser;

    laser.lifetime = laser.lifetime.saturating_sub(1);

    if laser.lifetime == 0 {
        return None;
    }

    let angle = laser.angle;
    let x = LASER_SPEED as f64 * angle.to_radians().cos();
    let y = LASER_SPEED as f64 * angle.to_radians().sin();
    laser.position = laser.position.offset(x as i32, y as i32);

    Some(laser)
}

#[derive(Debug, Clone, Copy)]
pub enum LaserSprite {
    Green,
}

impl Sprite for LaserSprite {
    fn get_src_rect(&self) -> Rect {
        use LaserSprite::*;
        let (x, y) = match self {
            Green => (32, 0),
        };

        Rect::new(x, y, SPRITE_WIDTH, SPRITE_HEIGHT)
    }
}

const LASER_SPEED: u32 = 60;
const LIFETIME: u32 = 40;

#[derive(Debug, Clone, PartialEq)]
pub struct Laser {
    position: Point,
    angle: f64,
    lifetime: u32,
}

impl Laser {
    pub fn new(position: Point, angle: f64) -> Self {
        Self { position, angle, lifetime: LIFETIME }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl Sprite for Laser {
    fn get_src_rect(&self) -> Rect {
        LaserSprite::Green.get_src_rect()
    }
}
