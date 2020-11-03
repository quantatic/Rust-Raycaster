use crate::World;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::Sdl;

use std::convert::TryInto;
use std::error::Error;

pub struct Window {
    width: usize,
    height: usize,
    block_size: usize,
    sdl2_canvas: Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(
        context: &Sdl,
        width: usize,
        height: usize,
        block_size: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = context.video()?;
        let window = video_subsystem
            .window(
                "Raycasting",
                (width * block_size) as u32,
                (height * block_size) as u32,
            )
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;

        Ok(Self {
            sdl2_canvas: canvas,
            width,
            height,
            block_size,
        })
    }

    pub fn update(&mut self, world: &World) -> Result<(), Box<dyn Error>> {
        self.sdl2_canvas.set_draw_color(Color::BLACK);
        self.sdl2_canvas.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                if world.block_at(x, y)? {
                    self.sdl2_canvas.set_draw_color(Color::GREEN);
                    self.sdl2_canvas.draw_rect(Rect::new(
                        (x * self.block_size).try_into()?,
                        (y * self.block_size).try_into()?,
                        self.block_size.try_into()?,
                        self.block_size.try_into()?,
                    ))?;
                }
            }
        }

        let mut angle: f64 = f64::to_degrees(world.get_user_angle()) - 30.0;
        while angle < f64::to_degrees(world.get_user_angle()) + 30.0 {
            let angle_rads = f64::to_radians(angle);

            let dist = world.cast(world.get_user_x(), world.get_user_y(), angle_rads);
            let end_x = world.get_user_x() + angle_rads.cos() * dist;
            let end_y = world.get_user_y() + angle_rads.sin() * dist;

            self.sdl2_canvas.set_draw_color(Color::GREEN);
            let start_point = Point::new(
                (world.get_user_x() * self.block_size as f64) as i32,
                (world.get_user_y() * self.block_size as f64) as i32,
            );
            let end_point = Point::new(
                (end_x * self.block_size as f64) as i32,
                (end_y * self.block_size as f64) as i32,
            );

            self.sdl2_canvas.draw_line(start_point, end_point)?;

            angle += 0.5;
        }

        self.sdl2_canvas.present();

        Ok(())
    }
}
