use rand::Rng;
use sdl2::rect::{Point, Rect};

use crate::{SdlCopy, Sprite, SCALE, SPRITE_HEIGHT, SPRITE_WIDTH, laser::{Laser, LaserSprite}};

/// Calculates the angle between the points, given the X axis as the second line
///
/// # Parameters
///
/// - `point_a` The first point
/// - `point_b` The second point
///
/// # Returns
///
/// An angle between 0 and 360 degrees
fn angle_between_points(point_a: Point, point_b: Point) -> f64 {
    let (x1, y1) = (point_a.x() as f64, point_a.y() as f64);
    let (x2, y2) = (point_b.x() as f64, point_b.y() as f64);

    let delta_x = x2 - x1;
    let delta_y = y2 - y1;

    // Calculate the angle using atan2 and convert it to degrees
    let angle_rad = delta_y.atan2(delta_x);
    let angle_deg = angle_rad.to_degrees();

    // Ensure the result is in the range 0..360]
    (angle_deg + 360.0) % 360.0
}

pub fn update_enemy(enemy: &Enemy, player_pos: Point, laser: &mut Vec<Laser>) -> Enemy {
    let mut enemy = enemy.clone();

    // Facing towards the player 
    let angle_between_ships = angle_between_points(enemy.position(), player_pos);
    enemy.angle = angle_between_ships;


    // Shooting lasers
    // FIX: Enemy will always fire at player
    let chance = rand::thread_rng().gen_range(0..100);
    if angle_between_ships == enemy.angle && chance > 95 {
        laser.push(Laser::new(enemy.position(), enemy.angle, LaserSprite::Red));
    }

    let distance_between = enemy.position - player_pos;
    let distance_between = ((distance_between.x.pow(2) + distance_between.y.pow(2)) as f64).sqrt();
    if distance_between < ENEMY_STOPPING_POINT {
        return enemy;
    }

    let angle = enemy.angle();
    let x = ENEMY_SPEED as f64 * angle.to_radians().cos();
    let y = ENEMY_SPEED as f64 * angle.to_radians().sin();
    enemy.position = enemy.position().offset(x as i32, y as i32);

    enemy
}

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

const ENEMY_SPEED: u32 = 10;
/// At which point the ships stop trying to fly closer to the player
const ENEMY_STOPPING_POINT: f64 = 200.0;

#[derive(Debug, Clone, PartialEq)]
/// A struct represnting the player's ship
pub struct Enemy {
    position: Point,
    angle: f64,
}

impl Enemy {
    pub fn new(position: Point) -> Self {
        Self {
            position,
            angle: 0.0,
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl SdlCopy for Enemy {
    fn get_src_rect(&self) -> Rect {
        EnemySprite::Stationary.get_src_rect()
    }

    fn get_dst_rect(&self, center_screen: Point) -> Rect {
        Rect::from_center(
            center_screen + self.position(),
            SCALE * SPRITE_WIDTH,
            SCALE * SPRITE_HEIGHT,
        )
    }
}
