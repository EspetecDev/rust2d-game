use crate::WIDTH;
use crate::HEIGHT;

pub struct Player {
    x: usize,
    y: usize,
    color: u32,
    size: usize,
    regular_speed: usize,
    boost_speed: usize,
    current_speed: usize,
}

pub enum DirState {
    Negative,
    Zero,
    Positive
}

impl Player {
    pub fn new(x: usize, y: usize, color: u32, size: usize, regular_speed: usize, boost_speed: usize) -> Self {
        Self { x, y, color, size, regular_speed, boost_speed, current_speed: regular_speed  }
    }

    pub fn draw(&self, buffer: &mut [u32]) {
        for row in 0..self.size {
            for col in 0..self.size {
                let px = self.x + col;
                let py = self.y + row;

                if px < WIDTH && py < HEIGHT {
                    buffer[py * WIDTH + px] = self.color;
                }
            }
        }
    }

    pub fn move_player(&mut self, x_dir: DirState, y_dir: DirState) {

        let mut new_x = self.x as isize;
        let mut new_y = self.y as isize;

        // pixel order goes from top left to bottom right
        match x_dir {
            DirState::Positive => new_x = new_x + self.current_speed as isize * -1,
            DirState::Negative => new_x = new_x + self.current_speed as isize,
            DirState::Zero => {}
        }
        match y_dir {
            DirState::Positive => new_y = new_y + self.current_speed as isize * -1,
            DirState::Negative => new_y = new_y + self.current_speed as isize,
            DirState::Zero => {}
        }

        // Check if new position would keep player fully within screen bounds
        if new_x >= 0 && (new_x as usize) + self.size <= WIDTH {
            self.x = new_x as usize;
        }
        if new_y >= 0 && (new_y as usize) + self.size <= HEIGHT {
            self.y = new_y as usize;
        }
    }

    pub fn boost(&mut self, activate: bool) {

        if activate { 
            self.current_speed = self.boost_speed; 
        }
        else { 
            self.current_speed = self.regular_speed; 
        }
    }   
}