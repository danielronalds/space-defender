use sdl2::rect::{Point, Rect};

use crate::{SdlCopy, Sprite, SPRITE_HEIGHT, SPRITE_WIDTH, SCALE};

pub fn update_laser(laser: Laser) -> Option<Laser> {
    let mut laser = laser;

    laser.lifetime = laser.lifetime.saturating_sub(1);

    if laser.lifetime == 0 {
        return None;
    }

    laser.old_position = laser.position;

    let laser_speed = match laser.color {
        LaserSprite::Green => GREEN_LASER_SPEED,
        LaserSprite::Red => RED_LASER_SPEED,
    };

    let angle = laser.angle;
    let x = laser_speed as f64 * angle.to_radians().cos();
    let y = laser_speed as f64 * angle.to_radians().sin();
    laser.position = laser.position.offset(x as i32, y as i32);

    Some(laser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaserSprite {
    Green,
    Red,
}

impl Sprite for LaserSprite {
    fn get_src_rect(&self) -> Rect {
        use LaserSprite::*;
        let (x, y) = match self {
            Green => (32, 0),
            Red => (80, 0)
        };

        Rect::new(x, y, SPRITE_WIDTH, SPRITE_HEIGHT)
    }
}

const GREEN_LASER_SPEED: u32 = 60;
const RED_LASER_SPEED: u32 = 30;
const LIFETIME: u32 = 40;

#[derive(Debug, Clone, PartialEq)]
pub struct Laser {
    color: LaserSprite,
    position: Point,
    old_position: Point,
    angle: f64,
    lifetime: u32,
}

impl Laser {
    pub fn new(position: Point, angle: f64, color: LaserSprite) -> Self {
        Self {
            position,
            old_position: position,
            color,
            angle,
            lifetime: LIFETIME,
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn hit(&self, center_screen: Point, rect: Rect) -> bool {
        rect.intersect_line(self.old_position + center_screen, self.position + center_screen)
            .is_some()
    }
}

impl SdlCopy for Laser {
    fn get_src_rect(&self) -> Rect {
        self.color.get_src_rect()
    }

    fn get_dst_rect(&self, center_screen: Point) -> Rect {
        Rect::from_center(
            center_screen + self.position(),
            SCALE * SPRITE_WIDTH,
            SCALE * SPRITE_HEIGHT,
        )
    }
}
