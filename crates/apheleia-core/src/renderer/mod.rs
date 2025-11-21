use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor, execute, queue,
    style::{PrintStyledContent, Stylize},
    terminal::Clear,
};

use crate::buffer::Buffer;

pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        let stdout = stdout();

        Self { stdout }
    }

    pub fn flip(mut self, buf: &mut Buffer) {
        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));

        for i in 0..(buf.height) {
            for j in 0..(buf.width) {
                queue!(
                    self.stdout,
                    cursor::MoveTo(j as u16, i as u16),
                    PrintStyledContent(buf.get(j, i).white())
                );
            }
        }

        self.stdout.flush();
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
