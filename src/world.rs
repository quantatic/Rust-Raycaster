use std::error::Error;

#[derive(Debug)]
pub struct World {
    blocks: Vec<Vec<bool>>, // access as blocks[y][x]
    width: usize,
    height: usize,
    user_x: f64,
    user_y: f64,
    user_angle: f64,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            blocks: vec![vec![false; width]; height],
            width,
            height,
            user_x: 0.0,
            user_y: 0.0,
            user_angle: 0.0,
        }
    }

    // angle is given in radians
    pub fn cast(&self, start_x: f64, start_y: f64, angle: f64) -> f64 {
        // first, check collisions with vertical part of blocks
        let mut shortest_distance: f64 = f64::MAX;

        let mut x = if angle.cos() > 0.0 {
            start_x.ceil()
        } else {
            start_x.floor()
        };

        let dx = angle.cos().signum();

        let slope = angle.tan();
        let dy = slope * dx;

        let mut y = start_y + slope * (x - start_x);

        while x >= 0.0 && x < (self.width as f64) && y >= 0.0 && y < (self.height as f64) {
            let lookup_x = (x + if dx < 0.0 { -1.0 } else { 0.0 }) as usize;
            let lookup_y = y as usize;

            if self.blocks[lookup_y][lookup_x] {
                break;
            }

            x += dx;
            y += dy;
        }

        shortest_distance = f64::min(
            shortest_distance,
            ((start_x - x).powi(2) + (start_y - y).powi(2)).sqrt(),
        );

        // now, check collisions with horizontal part of blocks
        let mut y = if angle.sin() > 0.0 {
            start_y.ceil()
        } else {
            start_y.floor()
        };

        let dy = angle.sin().signum();

        let slope = angle.tan();
        let dx = dy / slope;

        let mut x = start_x + (y - start_y) / slope;

        while x >= 0.0 && x < (self.width as f64) && y >= 0.0 && y < (self.height as f64) {
            let lookup_x = x as usize;
            let lookup_y = (y + if dy < 0.0 { -1.0 } else { 0.0 }) as usize;

            if self.blocks[lookup_y][lookup_x] {
                break;
            }

            x += dx;
            y += dy;
        }

        shortest_distance = f64::min(
            shortest_distance,
            ((start_x - x).powi(2) + (start_y - y).powi(2)).sqrt(),
        );

        shortest_distance
    }

    pub fn block_at(&self, x: usize, y: usize) -> Result<bool, Box<dyn Error>> {
        if x >= self.width || y >= self.height {
            Err(format!(
                "({}, {}) is out of bounds for world with width {} and height {}",
                x, y, self.width, self.height
            )
            .into())
        } else {
            Ok(self.blocks[y][x])
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, block: bool) -> Result<(), Box<dyn Error>> {
        if x >= self.width || y >= self.height {
            Err(format!(
                "({}, {}) is out of bounds for world with width {} and height {}",
                x, y, self.width, self.height
            )
            .into())
        } else {
            self.blocks[y][x] = block;
            Ok(())
        }
    }

    // angle is given in radians
    pub fn rotate_user(&mut self, delta_angle: f64) {
        self.user_angle += delta_angle;
    }

    pub fn move_user(&mut self, amount: f64) {
        self.user_x += amount * self.user_angle.cos();
        self.user_y += amount * self.user_angle.sin();
    }

    pub fn get_user_x(&self) -> f64 {
        self.user_x
    }

    pub fn get_user_y(&self) -> f64 {
        self.user_y
    }

    pub fn get_user_angle(&self) -> f64 {
        self.user_angle
    }
}
