pub mod cgol;
use cgol::CGOL;

pub mod performance;
use performance::Performance;
extern crate minifb;
use minifb::{CursorStyle, Key, KeyRepeat, MouseMode, Window, WindowOptions};
use rand::random;

const DEFAULT_COLOR: u32 = 0xFFFF0074;
const SIZE: (usize, usize) = (1280, 720);

fn main() {
    let mut world = CGOL::new(SIZE.0 as u32, SIZE.1 as u32);
    world.randomise();
    let mut window = Window::new(
        "Conway's Game of Life",
        SIZE.0,
        SIZE.1,
        WindowOptions{
            borderless: true,
            title: false,
            resize: false,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::Center,
            topmost: false,
            none: true,
            transparency: true,
        },
    )
    .unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16_660)));
    window.set_cursor_style(CursorStyle::Crosshair);
    world.color = DEFAULT_COLOR;

    let mut buffer = world.generate_draw_buffer();
    let mut is_playing = true;
    loop {
        if window.is_open() && window.is_key_down(Key::Escape) {
            break;
        }
        let is_shifting = window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift);
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| match (key, is_shifting) {
                (Key::R, true) => world.randomise(),
                (Key::C, true) => world.clear(),
                (Key::Space, false) => is_playing = !is_playing,
                (Key::Right, false) => {
                    if !is_playing {
                        world.tick()
                    }
                }
                _ => (),
            });
        if is_playing {
            world.tick();
        }
        window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
            let noise = (random::<f32>() * 10.0 + 40.0) as u32;
            world.draw_at(mouse.0 as u32, mouse.1 as u32, 1, noise);
        });
        buffer = world.draw_and_clear_buffer(buffer);
        window.update_with_buffer(&buffer, SIZE.0, SIZE.1).unwrap();
    }
}
