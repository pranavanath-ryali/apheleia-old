use std::io::{stdout, Write};
use crossterm::{cursor, queue, style::{self, Stylize}, terminal::{self, Clear}, ExecutableCommand, QueueableCommand};

pub fn setup() {
    let size = terminal::size();
    if let Ok(s) = size {
        println!("{} {}", s.0, s.1);
    }

    let mut stdout = stdout();

    stdout.execute(Clear(terminal::ClearType::All));

    queue!(stdout,
        cursor::MoveTo(0, 0),
        style::PrintStyledContent("A".red())
    );

    // stdout
    //     .queue(Clear(terminal::ClearType::All));
    //
    // queue!(stdout, cursor::MoveTo(5, 5));



    stdout.flush();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
