use std::io::{self, Write};

use crossterm::{
    style,
    event,
    cursor,
    QueueableCommand,
    terminal,
};

use ndarray::{Array, Ix2, AssignElem};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .queue(event::EnableMouseCapture)?
        .queue(cursor::Hide)?;

    stdout.flush()?;

    let size = terminal::size()?;
    let mut screen = ScreenBuffer::new([size.0 as usize, size.1 as usize]);
    screen.buffer.fill(127);
    screen.draw(&mut stdout)?;

    loop {
        match event::read()? {
            event::Event::Key(_) => {break;},
            event::Event::Mouse(e) => if let event::MouseEventKind::Drag(_) = e.kind {
                screen.buffer.get_mut((e.column as usize, e.row as usize)).unwrap().assign_elem(128);
                screen.draw(&mut stdout)?;
            },
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
        stdout.queue(cursor::MoveTo(0, 0))?;
        for row in self.buffer.columns() {
            for col in row {
                if *col > 127 {
                    stdout.queue(style::Print('.'))?;
                } else {
                    stdout.queue(style::Print(' '))?;
                }
            }
            stdout
                .queue(cursor::MoveToNextLine(1))?;
        }
        stdout.flush()?;
        Ok(())
    }
}
