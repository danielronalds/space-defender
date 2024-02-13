use sdl2::rect::{Point, Rect};

use crate::{SPRITE_HEIGHT, SPRITE_WIDTH};

/// Updates the player based on the frame tick
pub fn update_player(player: &mut Player) {
    if player.thrusters() {
        if player.speed() < MAX_SPEED {
            player.set_speed(player.speed() + ACCELERATION);
        }
    } else {
        player.set_speed(player.speed().saturating_sub(DECCELERATION));
    }

    let player_agility = match player.thrusters() {
        true => AGILITY,
        false => AGILITY / 2.0,
    };

    if player.rotating_left() {
        player.set_angle((player.angle() - player_agility) % 365.0);
    }

    if player.rotating_right() {
        player.set_angle((player.angle() + player_agility) % 365.0);
    }

    let angle = player.angle();
    let x = player.speed() as f64 * angle.to_radians().cos();
    let y = player.speed() as f64 * angle.to_radians().sin();
    player.set_position(player.position().offset(x as i32, y as i32));

}

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

const MAX_SPEED: u32 = 20;
const ACCELERATION: u32 = 3;
const DECCELERATION: u32 = 1;
const AGILITY: f64 = 7.0;

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
