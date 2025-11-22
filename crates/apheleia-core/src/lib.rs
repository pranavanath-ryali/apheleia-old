pub mod buffer;
pub mod renderer;

use std::thread::{sleep, sleep_ms};

use crate::{buffer::Buffer, renderer::Renderer};
use crossterm::
    terminal::{self}
;

pub fn setup() {
    let size = terminal::size().unwrap();
    let mut buf = Buffer::new(size.0 as usize, size.1 as usize);
    let mut renderer = Renderer::new();

    buf.write_line(0, 0, "+");
    buf.write_line(10, 3, "Hello World!");

    renderer.flip(&mut buf);
}
