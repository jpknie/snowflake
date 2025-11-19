//@Author Jani Nieminen, 2025.. maybe some christmas demo

use pixel_engine::{PixEngine, PixelBuffer, Scene};
use rand::{prelude::*, random_range};
use std::{mem::swap, thread::sleep, time::Duration};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const N_SNOW: usize = 2;


struct SnowyScreen {
    current_grid: Vec<Vec<u32>>,
    next_grid: Vec<Vec<u32>>
}

impl SnowyScreen {
    fn new() -> Self {
        let mut rng = rand::rng();
        let snow: Vec<u32> = (0..N_SNOW).map(|_| Self::spawn(&mut rng)).collect();
        let mut init_v = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
        for s in snow.iter() {
            init_v[0][*s as usize] = 1;
        }

        SnowyScreen { current_grid: init_v, next_grid: vec![vec![0; WIDTH as usize]; HEIGHT as usize] }
        
    }

    fn spawn<R: Rng + ?Sized>(rng: &mut R) -> u32 {
        let x = rng.random_range(0..WIDTH) as u32; 
        x
    }

    fn generate_snow(&mut self) {
        let mut rng = rand::rng();
        let snow: Vec<u32> = (0..N_SNOW).map(|_| Self::spawn(&mut rng)).collect();
        for s in snow.iter() {
            self.current_grid[0][*s as usize] = 1;
        }
    }

    fn swap_buffers(&mut self) {
        swap(&mut self.current_grid, &mut self.next_grid);
    }
   
}

impl Scene for SnowyScreen {
    fn update(&mut self, dt: f64, _fb: &mut PixelBuffer) {
        for y in 0..HEIGHT {
            // Empty the new buffer
            for x in 0..WIDTH {
                self.next_grid[y as usize][x as usize] = 0;
            }
        }

        for dy in (0..HEIGHT).rev() {
            for dx in 0..WIDTH {            
                if self.current_grid[dy as usize][dx as usize] == 1 {
                    let deltas: Vec<(i32, i32)> = vec![(-1,1),(1,1),(0,1)];
                    let mut rng = rand::rng();
                    let num = rng.random_range(0..3);
                    let chosen_delta = deltas[num];
                    let nx = dx as i32 + chosen_delta.0;
                    let ny = dy as i32 + chosen_delta.1;
                    if ny < HEIGHT as i32 
                    && nx < WIDTH as i32
                    && nx >= 0
                    && ny >= 0 
                    && self.current_grid[ny as usize][nx as usize] == 0 {
                        self.next_grid[ny as usize][nx as usize] = 1;
                    } else {
                        self.next_grid[dy as usize][dx as usize] = 1;
                    }
                    
                }
            }
        }
        self.swap_buffers();
        self.generate_snow();
    }

    fn draw(&self, fb: &mut PixelBuffer) {
        fb.clear([0, 0, 0, 0]);
        for dy in 0..HEIGHT {
            for dx in 0..WIDTH {
                if self.current_grid[dy as usize][dx as usize] == 1 {
                    fb.put(dx as i32, dy as i32, [0xff,0xff,0xff,0xff]);
                }
            }
        }
        
    }
}

fn main() {
    let scene = SnowyScreen::new();
    let mut engine = PixEngine::new(WIDTH as u32, HEIGHT as u32, false, "Starfield", scene);
    engine.run();
}
