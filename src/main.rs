use std::time::Duration;

use enemy::{update_enemy, Enemy};
use laser::{update_laser, Laser, LaserSprite};
use player::{update_player, Player};
use rand::Rng;
use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

trait Sprite {
    fn get_src_rect(&self) -> Rect;
}

/// A trait that defines the required methods for a struct to be copied to the canvas
trait SdlCopy {
    /// The src rectangle from the pallete
    fn get_src_rect(&self) -> Rect;

    /// The dst rectangle. Offset of the screen center should already be factored in
    fn get_dst_rect(&self, center_screen: Point) -> Rect;
}

mod player;

mod laser;

mod enemy;

const SPRITE_WIDTH: u32 = 16;
const SPRITE_HEIGHT: u32 = 16;

const SCALE: u32 = 3;

type SdlError = Result<(), String>;

fn render(
    canvas: &mut WindowCanvas,
    center_screen: Point,
    texture: &Texture,
    player: &Player,
    lasers: &[Laser],
    enemies: &[Enemy],
    enemy_lasers: &[Laser],
) -> SdlError {
    canvas.clear();

    // Rendering Player
    canvas.copy_ex(
        texture,
        player.get_src_rect(),
        player.get_dst_rect(center_screen),
        // Below we're adding 90 degrees so that the movement lines up with what is happening
        (player.angle() + 90.0) % 360.0,
        None,
        false,
        false,
    )?;

    // Rendering Enemies
    for enemy in enemies {
        canvas.copy_ex(
            texture,
            enemy.get_src_rect(),
            enemy.get_dst_rect(center_screen),
            (enemy.angle() + 90.0) % 360.0,
            None,
            false,
            false,
        )?;
    }

    // Rendering Lasers
    for laser in lasers {
        canvas.copy_ex(
            texture,
            laser.get_src_rect(),
            laser.get_dst_rect(center_screen),
            (laser.angle() + 90.0) % 360.0,
            None,
            false,
            false,
        )?;
    }

    for laser in enemy_lasers {
        canvas.copy_ex(
            texture,
            laser.get_src_rect(),
            laser.get_dst_rect(center_screen),
            (laser.angle() + 90.0) % 360.0,
            None,
            false,
            false,
        )?;
    }

    canvas.present();
    Ok(())
}

fn update(
    player: &mut Player,
    lasers: &[Laser],
    enemies: &[Enemy],
    enemy_lasers: &mut Vec<Laser>,
    center_screen: Point,
) -> (Vec<Laser>, Vec<Enemy>, Vec<Laser>) {
    update_player(player);

    let lasers = lasers
        .iter()
        .filter_map(|l| update_laser(l.clone()))
        .collect::<Vec<Laser>>();

    let mut enemy_lasers = enemy_lasers
        .iter()
        .filter_map(|l| update_laser(l.clone()))
        .collect::<Vec<Laser>>();

    let enemies = enemies
        .into_iter()
        .filter_map(|e| {
            for laser in lasers.iter() {
                if laser.hit(center_screen, e.get_dst_rect(center_screen)) {
                    return None;
                }
            }
            Some(update_enemy(e, player.position(), &mut enemy_lasers, &enemies))
        })
        .collect();

    (lasers, enemies, enemy_lasers)
}

fn main() -> SdlError {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("Space Defender", 1920, 1080)
        .position_centered()
        .build()
        .expect("Failed to build window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to build canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/sprites.png")?;

    let mut player = Player::default();
    let mut lasers = vec![];

    let mut enemies = vec![Enemy::new(Point::new(500, 500))];
    let mut enemy_lasers = vec![];

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player.set_thrusters(true);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player.set_thrusters(false);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    player.set_rotating_right(true);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    player.set_rotating_left(true);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    player.set_rotating_right(false);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    player.set_rotating_left(false);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    lasers.push(Laser::new(
                        player.position(),
                        player.angle(),
                        LaserSprite::Green,
                    ));
                }
                _ => (),
            }
        }

        let (width, height) = canvas.output_size()?;
        let center_screen = Point::new(width as i32 / 2, height as i32 / 2);

        let (new_lasers, new_enemies, new_enemy_lasers) = update(
            &mut player,
            &lasers,
            &enemies,
            &mut enemy_lasers,
            center_screen,
        );
        lasers = new_lasers;
        enemies = new_enemies;
        enemy_lasers = new_enemy_lasers;

        // TODO: Remove this when not testing
        while enemies.len() < 7 {
            let mut rng = rand::thread_rng();
            let spawn = Point::new(
                rng.gen_range(0..=width) as i32,
                rng.gen_range(0..=height) as i32,
            );
            enemies.push(Enemy::new(spawn - center_screen));
        }

        render(
            &mut canvas,
            center_screen,
            &texture,
            &player,
            &lasers,
            &enemies,
            &enemy_lasers,
        )?;

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
    }

    Ok(())
}
