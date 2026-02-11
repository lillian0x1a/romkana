use romkana::RomKana;
use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, Hide, Show},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let engine = match RomKana::new("romaji.csv") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load table: {}", e);
            return Err(e);
        }
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, Hide)?;

    loop {
        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        print!("Exit  : Esc");
        execute!(stdout, MoveTo(0, 1))?;
        print!("Input : {}", input);
        execute!(stdout, MoveTo(0, 2))?;
        print!("Change: {}", engine.convert(&input));

        stdout.flush()?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => { input.pop();},
                KeyCode::Esc => break,
                _ =>{}
            }
        }
    }
    execute!(stdout, Show)?;
    disable_raw_mode()?;
    Ok(())
}
