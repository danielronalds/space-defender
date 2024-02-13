use std::time::Duration;

use player::{Player, update_player};
use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

mod player;

const SPRITE_WIDTH: u32 = 16;
const SPRITE_HEIGHT: u32 = 16;

const SCALE: u32 = 6;

type SdlError = Result<(), String>;

fn render(canvas: &mut WindowCanvas, texture: &Texture, player: &Player) -> SdlError {
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let center_screen = Point::new(width as i32 / 2, height as i32 / 2);

    let screen_rect = Rect::from_center(
        center_screen + player.position(),
        SCALE * SPRITE_WIDTH,
        SCALE * SPRITE_HEIGHT,
    );

    canvas.copy_ex(
        texture,
        player.get_src_rect(),
        screen_rect,
        // Below we're adding 90 degrees so that the movement lines up with what is happening
        (player.angle() + 90.0) % 365.0,
        None,
        false,
        false,
    )?;

    canvas.present();
    Ok(())
}

fn update(player: &mut Player) {
    update_player(player);
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
                _ => (),
            }
        }

        update(&mut player);

        render(&mut canvas, &texture, &player)?;

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
    }

    Ok(())
}
