pub mod buffer;
pub mod renderer;

use crate::{buffer::Buffer, renderer::Renderer};
use crossterm::
    terminal::{self}
;

pub fn setup() {
    let size = terminal::size().unwrap();
    let mut buf = Buffer::new_fill(size.0 as usize, size.1 as usize, 'a');
    let renderer = Renderer::new();

    renderer.flip(&mut buf);

    // stdout.execute(Clear(terminal::ClearType::All));
    //
    // for i in 0..(size.0) {
    //     for j in 0..(size.1) {
    //         stdout.execute(cursor::MoveTo(0, 0));
    //     }
    // }

    //     println!("{} {}", s.0, s.1);
    //
    //     let mut stdout = stdout();
    //
    //     stdout.execute(Clear(terminal::ClearType::All));
    //
    //     for i in 0..(s.0 - 0) {
    //         // queue!(
    //         //     stdout,
    //         //     cursor::MoveTo(i, 0),
    //         //     style::PrintStyledContent("2".red())
    //         // );
    //         for j in 0..(s.1 - 0) {
    //             queue!(
    //                 stdout,
    //                 cursor::MoveTo(i, j),
    //                 style::PrintStyledContent("2".red())
    //             );
    //         }
    //     }
    //
    //     stdout.flush();
    // }
    // stdout
    //     .queue(Clear(terminal::ClearType::All));
    //
    // queue!(stdout, cursor::MoveTo(5, 5));
}
