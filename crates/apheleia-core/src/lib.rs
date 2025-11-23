pub mod buffer;
pub mod renderer;

use std::thread::sleep_ms;

use crate::{buffer::Buffer, renderer::Renderer};
use crossterm::terminal::{self};

pub fn setup() {
    let size = terminal::size().unwrap();
    let mut buf = Buffer::new_fill(size.0 - 10, size.1 - 20, 'c');
    let mut renderer = Renderer::new();

    buf.write_line(0, 0, "♥");
    buf.write_line(10, 3, "♥ Hello World!");

    renderer.flip(&mut buf);
    
    let mut i: u16 = 1;
    buf.write_line(0, 0, "HI");
    loop {
        sleep_ms(1000);
        buf.write_line(i - 1, i - 1, "  ");
        buf.write_line(i, i, "HI");

        renderer.update(&mut buf);
        i += 1;
    }

    renderer.quit();
}
