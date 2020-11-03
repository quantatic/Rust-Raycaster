use crate::World;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::Sdl;

use std::convert::TryInto;
use std::error::Error;

// const PROJECTION_PLANE_DISTANCE: f64 = 1.0;

pub struct Window {
    width: usize,
    height: usize,
    block_size: usize,
    sdl2_canvas: Canvas<sdl2::video::Window>,
    fov: f64,
}

impl Window {
    pub fn new(
        context: &Sdl,
        width: usize,
        height: usize,
        block_size: usize,
        fov: f64,
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
            fov,
        })
    }

    pub fn update(&mut self, world: &World) -> Result<(), Box<dyn Error>> {
        self.sdl2_canvas.set_draw_color(Color::BLACK);
        self.sdl2_canvas.clear();

        // draw block squares
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

        // draw actual slices

        let mut slice_x = 0;
        self.sdl2_canvas.set_draw_color(Color::BLUE);
        while slice_x < (self.width * self.block_size) {
            let angle = world.get_user_angle()
                + (self.fov * (((slice_x as f64) / (self.width * self.block_size) as f64) - 0.5));
            let delta_angle = angle - world.get_user_angle();

            let dist = world.cast(world.get_user_x(), world.get_user_y(), angle);

            let slice_height =
                (((self.height * self.block_size) as f64) / (dist * delta_angle.cos())) as usize;
            let slice_whitespace =
                ((self.height * self.block_size).saturating_sub(slice_height)) / 2; // saturating sub, as whitespace is at least 0 (never underflows)

            let start_point = Point::new(slice_x as i32, slice_whitespace as i32);

            let end_point = Point::new(
                slice_x as i32,
                ((self.height * self.block_size) - slice_whitespace) as i32,
            );

            self.sdl2_canvas.draw_line(start_point, end_point)?;

            slice_x += 1;
        }

        // draw ray lines
        let mut angle = world.get_user_angle() - (self.fov / 2.0);
        let delta_angle = self.fov / ((self.width * self.block_size) as f64);
        while angle < world.get_user_angle() + (self.fov / 2.0) {
            let dist = world.cast(world.get_user_x(), world.get_user_y(), angle);
            let end_x = world.get_user_x() + angle.cos() * dist;
            let end_y = world.get_user_y() + angle.sin() * dist;

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

            angle += delta_angle;
        }

        // draw user
        self.sdl2_canvas.set_draw_color(Color::RED);
        self.sdl2_canvas.draw_rect(Rect::new(
            ((world.get_user_x() - 0.5) * self.block_size as f64) as i32,
            ((world.get_user_y() - 0.5) * self.block_size as f64) as i32,
            self.block_size.try_into()?,
            self.block_size.try_into()?,
        ))?;

        self.sdl2_canvas.present();
        Ok(())
    }
}
