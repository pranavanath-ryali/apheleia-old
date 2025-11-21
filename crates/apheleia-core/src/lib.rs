use crossterm::{
    ExecutableCommand, QueueableCommand, cursor, queue,
    style::{self, Stylize},
    terminal::{self, Clear},
};
use std::io::{Write, stdout};

pub fn setup() {
    let size = terminal::size();
    if let Ok(s) = size {
        println!("{} {}", s.0, s.1);

        let mut stdout = stdout();

        stdout.execute(Clear(terminal::ClearType::All));

        // queue!(
        //     stdout,
        //     cursor::MoveTo(3, 0),
        //     style::PrintStyledContent("A".red())
        // );
        // queue!(
        //     stdout,
        //     cursor::MoveTo(5, 0),
        //     style::PrintStyledContent("123".red())
        // );
        //
        for i in 0..(s.0 - 0) {
            // queue!(
            //     stdout,
            //     cursor::MoveTo(i, 0),
            //     style::PrintStyledContent("2".red())
            // );
            for j in 0..(s.1 - 0) {
                queue!(
                    stdout,
                    cursor::MoveTo(i, j),
                    style::PrintStyledContent("2".red())
                );
            }
        }

        stdout.flush();
    }
    // stdout
    //     .queue(Clear(terminal::ClearType::All));
    //
    // queue!(stdout, cursor::MoveTo(5, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
