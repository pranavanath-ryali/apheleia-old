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
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide);

        Self { stdout }
    }

    pub fn flip(&mut self, buf: &mut Buffer) {
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
        buf.clear_update_list();
    }

    pub fn update(&mut self, buf: &mut Buffer) {
        for cell in buf.get_update_list() {
            queue!(
                self.stdout,
                cursor::MoveTo(cell.0 as u16, cell.1 as u16),
                PrintStyledContent(cell.2.white())
            );
        }
        
        self.stdout.flush();
        buf.clear_update_list();
    }

    pub fn quit(&mut self) {
        execute!(self.stdout, cursor::Show);
    }
}
