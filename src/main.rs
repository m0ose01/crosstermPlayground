use std::io::{self, Write};

use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event,
    cursor,
    QueueableCommand,
    terminal,
};

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(cursor::MoveTo(0, 0))?
        .queue(cursor::Hide)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(SetForegroundColor(Color::DarkRed))?
        .queue(SetBackgroundColor(Color::DarkGrey))?
        .queue(Print("hello, world\n"))?
        .queue(ResetColor)?;

    stdout.flush()?;

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
