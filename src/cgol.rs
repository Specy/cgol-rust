use std::mem;
use rand::random;

#[non_exhaustive]
struct Cell;
impl Cell {
    pub const DEAD: u8 = 0;
    pub const ALIVE: u8 = 1;
}

pub struct CGOL {
    width: u32,
    height: u32,
    cells: Vec<u8>,
    buff: Vec<u8>,
    pub color: u32,
}

impl CGOL {
    pub fn new(width: u32, height: u32) -> CGOL {
        let cells = vec![Cell::DEAD; (width * height) as usize];
        let buff = vec![Cell::DEAD; (width * height) as usize];
        CGOL {
            width,
            height,
            cells,
            buff,
            color: 0xFFFFFFFF,
        }
    }   
    pub fn randomise(&mut self) {
        for i in 1..self.height - 1{
            for j in 1..self.width - 1{
                if random::<f32>() > 0.5 {
                    self.cells[(i * self.width + j) as usize] = Cell::ALIVE
                }
            }
        }
    }

    pub fn draw_at(&mut self, x_position : u32, y_position: u32, state: u8, noise: u32) {
        for _ in 0..noise{
            let x = ((random::<f32>() * -20.0 + 15.0) as u32)  + x_position;
            let y = ((random::<f32>() * -20.0 + 15.0) as u32)  + y_position;
            if x > 1 && y > 1 && x < self.width - 1 && y < self.height - 1 {
                let index = (y * self.width + x) as usize;
                self.cells[index] = state;
            }
        }

    }
    pub fn generate_draw_buffer(&mut self) -> Vec<u32> {
        let buffer: Vec<u32> = vec![0; (self.width * &self.height) as usize];
        buffer
    }
    pub fn draw_on_buffer(&mut self, mut buff: Vec<u32>) -> Vec<u32> {
        for (i,cell) in self.cells.iter().enumerate() {
            if Cell::ALIVE == *cell {
                buff[i] = self.color;
            }
        }
        buff
    }
    pub fn draw_and_clear_buffer(&mut self, mut buff: Vec<u32>) -> Vec<u32> {
        for (i,cell) in self.cells.iter().enumerate() {
            if Cell::ALIVE == *cell {
                buff[i] = self.color;
            }else{
                buff[i] = 0;
            }
        }
        buff
    }
    pub fn tick(&mut self) {
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
    pub fn clear(&mut self){
        for i in 0..self.cells.len() {
            self.cells[i] = Cell::DEAD;
            self.buff[i] = Cell::DEAD;
        }
    }
}