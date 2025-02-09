use std::io::{self, Write};

use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event,
    cursor,
    QueueableCommand,
    terminal,
};

use ndarray::{Array, Ix2};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .queue(cursor::Hide)?;

    stdout.flush()?;

    let size = terminal::size()?;
    let mut screen = ScreenBuffer::new([size.0 as usize, size.1 as usize]);
    screen.buffer.map_mut(|col| *col = 128);
    screen.draw(&mut stdout)?;

    loop {
        match event::read()? {
            event::Event::Key(event) => {println!("{:?}", event);break;},
            _ => (),
        }
    }

    stdout
        .queue(cursor::Show)?
        .queue(terminal::LeaveAlternateScreen)?
        .flush()?;

    terminal::disable_raw_mode()?;

    Ok(())
}

struct ScreenBuffer {
    buffer: Array<u8, Ix2>,
}

impl ScreenBuffer {
    fn new(size: [usize; 2]) -> Self {
        Self {
            buffer: Array::<u8, Ix2>::zeros(size),
        }
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        for row in self.buffer.columns() {
            for col in row {
                if *col > 127 {
                    stdout.queue(Print('.'))?;
                } else {
                    stdout.queue(Print(' '))?;
                }
            }
            stdout
                .queue(cursor::MoveToNextLine(1))?;
        }
        stdout.flush()?;
        Ok(())
    }
}
