use std::mem;
use std::thread;
use rand::random;
use std::time::Instant;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

#[non_exhaustive]
struct Cell;

impl Cell {
    pub const DEAD: u8 = 0;
    pub const ALIVE: u8 = 1;
}

struct CGOL {
    width: u32,
    height: u32,
    cells: Vec<u8>,
    buff: Vec<u8>,
}

impl CGOL {
    fn new(width: u32, height: u32) -> CGOL {
        let cells = vec![Cell::DEAD; (width * height) as usize];
        let buff = vec![Cell::DEAD; (width * height) as usize];
        CGOL {
            width,
            height,
            cells,
            buff,
        }
    }
    fn randomise(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = if random::<f32>() < 0.5 {
                Cell::ALIVE
            } else {
                Cell::DEAD
            };
        }
    }
    fn tick(&mut self) {
        let new_gen = &mut self.buff;
        let old_gen = &self.cells;
        let width = self.width;
        let height = self.height;
        for i in 1..height-1 {
            let rows = (((i - 1) * width) as usize,(i * width) as usize, ((i + 1) * width) as usize);
            for col in 1..width-1{
                let columns = ((col - 1) as usize, col as usize, (col + 1) as usize);
                let neighbours = 
                    old_gen[rows.0 + columns.0] +
                    old_gen[rows.0 + columns.1] +
                    old_gen[rows.0 + columns.2] +
                    old_gen[rows.1 + columns.0] +
                    old_gen[rows.1 + columns.2] +
                    old_gen[rows.2 + columns.0] +
                    old_gen[rows.2 + columns.1] +
                    old_gen[rows.2 + columns.2];
                new_gen[rows.1 + columns.1] = match (old_gen[rows.1 + columns.1], neighbours){
                    (Cell::ALIVE, neighbours) if neighbours < 2 => Cell::DEAD,
                    (Cell::ALIVE, 2) | (Cell::ALIVE, 3) => Cell::ALIVE,
                    (Cell::ALIVE, neighbours) if neighbours > 3 => Cell::DEAD,
                    (Cell::DEAD, 3) => Cell::ALIVE,
                    (_, _) => Cell::DEAD
                };
            }
        }
        mem::swap(&mut self.cells, &mut self.buff);
    }
}
fn main() {
    let size = (1920 as usize, 1080 as usize);
    let mut world = CGOL::new(size.0 as u32, size.1 as u32);
    world.randomise();

    let mut window = Window::new(
        "Conway's Game of Life",
        size.0,
        size.1,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut buffer: Vec<u32> = vec![0; size.0 * size.1];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        world.tick();
        let mut position = 0;
        for cell in world.cells.iter() {
            match *cell {
                Cell::ALIVE => buffer[position] = 0xFFFF0074,
                Cell::DEAD => buffer[position] = 0xFF0F0D19,
                _ => buffer[position] = 0xFF0F0D19,
            }
            position += 1;
        }

        window.update_with_buffer(&buffer, size.0 , size.1)
            .unwrap();
    }
}

struct Performance{
    start: Instant,
}
impl Performance{
    fn new() -> Performance{
        Performance{
            start: Instant::now()
        }
    }
    fn reset(&mut self){
        self.start = Instant::now();
    }
    fn end(&self,message: &str){
        let end = &self.start.elapsed();
        println!("{} > {:.2?}", message, end);
    }
}