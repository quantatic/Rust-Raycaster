use raycast::Window;
use raycast::World;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::error::Error;
use std::time::Duration;

const VIEW_FOV_DEGREES: f64 = 60.0;
const VIEW_FOV: f64 = VIEW_FOV_DEGREES * std::f64::consts::PI / 180.0;

const WORLD_WIDTH: usize = 30;
const WORLD_HEIGHT: usize = 30;

const WORLD_BLOCK_SIZE: usize = 30;

const DELTA_ANGLE: f64 = 0.03;
const MOVE_SPEED: f64 = 0.2;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;

    let mut window = Window::new(
        &sdl_context,
        WORLD_WIDTH,
        WORLD_HEIGHT,
        WORLD_BLOCK_SIZE,
        VIEW_FOV,
    )?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);
    for i in 5..15 {
        world.set_block(i, i + 10, true)?;
        world.set_block(i + 10, i, true)?;
    }

    for y in 0..WORLD_HEIGHT {
        world.set_block(0, y, true)?;
        world.set_block(WORLD_WIDTH - 1, y, true)?;
    }

    for x in 0..WORLD_WIDTH {
        world.set_block(x, 0, true)?;
        world.set_block(x, WORLD_HEIGHT - 1, true)?;
    }

    //world.set_block(5, 5, true);

    'running: loop {
        // poll for user exit
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for keycode in event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
        {
            match keycode {
                Keycode::Left => world.rotate_user(-DELTA_ANGLE),
                Keycode::Right => world.rotate_user(DELTA_ANGLE),
                Keycode::Up => world.move_user(MOVE_SPEED),
                Keycode::Down => world.move_user(-MOVE_SPEED),
                _ => {}
            };
        }

        window.update(&world)?;

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
