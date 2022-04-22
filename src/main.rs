use std::time::Instant;

pub mod cgol;
use cgol::CGOL;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
const DEFAULT_PALETTE: [u32; 4] = [0xFF283049, 0xFF404B69, 0xFF278EA5, 0xFF134753];
const DEFAULT_COLOR: u32 = 0xFFFF0074;
const SIZE: (usize,usize) = (800, 800);

fn main() {

    let mut world = CGOL::new(SIZE.0 as u32, SIZE.1 as u32);
    world.randomise();
    let mut window = Window::new(
        "Conway's Game of Life",
        SIZE.0,
        SIZE.1,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(32_666)));
    world.color = DEFAULT_COLOR;
    let mut i: u64 = 0;
    let mut color_index = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        world.tick();
        let mut buffer = world.generate_draw_buffer();
        buffer = world.draw_on_buffer(buffer);
        window.update_with_buffer(&buffer, SIZE.0 , SIZE.1)
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