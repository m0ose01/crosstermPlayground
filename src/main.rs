use std::io::stdout;

use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};

fn main() -> std::io::Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::DarkRed))?
        .execute(SetBackgroundColor(Color::DarkGrey))?
        .execute(Print("hello, world\n"))?
        .execute(ResetColor)?;
    Ok(())
}
