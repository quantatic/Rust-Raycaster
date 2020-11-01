use raycast::World;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use std::convert::TryInto;
use std::error::Error;
use std::time::Duration;

const WORLD_WIDTH: usize = 40;
const WORLD_HEIGHT: usize = 40;

const WORLD_BLOCK_SIZE: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Raycasting",
            (WORLD_WIDTH * WORLD_BLOCK_SIZE) as u32,
            (WORLD_HEIGHT * WORLD_BLOCK_SIZE) as u32,
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut i = 0;

    let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);
    for i in 5..15 {
        world.set_block(i, i + 10, true);
        world.set_block(i + 10, i, true);
    }

    //world.set_block(5, 5, true);

    let mut user_x = 0.0;
    let mut user_y = 0.0;

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

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

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                if world.block_at(x, y)? {
                    canvas.set_draw_color(Color::GREEN);
                    canvas.draw_rect(Rect::new(
                        (x * WORLD_BLOCK_SIZE).try_into()?,
                        (y * WORLD_BLOCK_SIZE).try_into()?,
                        WORLD_BLOCK_SIZE.try_into()?,
                        WORLD_BLOCK_SIZE.try_into()?,
                    ));
                }
            }
        }

        println!("{:?}", world);

        let mut angle: f64 = 0.0;
        while angle < 360.0 {
			let angle_rads = f64::to_radians(angle);

            let dist = world.cast(user_x, user_y, angle_rads.into());
            let end_x = user_x + angle_rads.cos() * dist;
            let end_y = user_y + angle_rads.sin() * dist;

            canvas.set_draw_color(Color::GREEN);
            let start_point = Point::new(
                (user_x * WORLD_BLOCK_SIZE as f64) as i32,
                (user_y * WORLD_BLOCK_SIZE as f64) as i32,
            );
            let end_point = Point::new(
                (end_x * WORLD_BLOCK_SIZE as f64) as i32,
                (end_y * WORLD_BLOCK_SIZE as f64) as i32,
            );

            canvas.draw_line(start_point, end_point);

            angle += 5.0;
        }

        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(Rect::new(
            ((user_x - 0.5) * WORLD_BLOCK_SIZE as f64) as i32,
            ((user_y - 0.5) * WORLD_BLOCK_SIZE as f64) as i32,
            WORLD_BLOCK_SIZE.try_into()?,
            WORLD_BLOCK_SIZE.try_into()?,
        ));

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));

        user_x += 0.1;
        user_y += 0.1;
    }

    Ok(())
}
