use std::io::{Write, stdout};
use std::{
    self,
    fmt,
    error::Error, 
    env, 
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    cursor
};

pub struct Config {
    pub single_player: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // this really should be a flag, but right now
        // if they put an arg, its multiplayer
        let single_player = args.next().is_none();
        
        Ok(Config{ single_player })
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty = 0,
    X = 1,
    O = -1,
}

struct Board {
    array: [Tile; 9]
}

impl Board {
    fn new() -> Self {
        Self {
            array: [Tile::Empty; 9]
        }
    }

    fn set_tile(&mut self, tile: Tile, index: usize) {
        self.array[index] = tile;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display_str = String::new();
        for (i, tile) in self.array.iter().enumerate() {
            let col = i % 3;

            display_str.push(match tile {
                Tile::Empty => '*',
                Tile::X => 'X',
                Tile::O => 'O',
            });

            if col == 2 {
                display_str.push_str("\r\n");
            }
        }
        write!(f, "{}", display_str)
    }
}

/**
 * This function works, but the stuff updates too quickly
 * 
 */
fn log(s: &str) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(cursor::SavePosition)?;
    stdout.execute(cursor::MoveTo(10, 0))?;
    write!(&mut stdout, "{}", s)?;
    stdout.flush()?;
    stdout.execute(cursor::RestorePosition)?;

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut board = Board::new();
    let mut is_x = true;
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.flush()?;
    stdout.lock();

    stdout.execute(cursor::MoveTo(0, 0))?;

    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::SavePosition)?;
        stdout.execute(cursor::MoveTo(0,0))?;
        write!(&mut stdout, "{}", board)?;
        stdout.flush()?;
        stdout.execute(cursor::RestorePosition)?;
        if let Event::Key(event) = event::read()? {
            match event.code { 
                KeyCode::Char(c @ '1'..='9') => {
                    if let Some(num) = c.to_digit(10) {
                        let tile = if is_x { Tile::X } else { Tile::O };
                        board.set_tile(tile, (num - 1) as usize);
                        is_x = !is_x;
                    }
                }, 
                KeyCode::Esc => break,
                // Maybe redundant
                _ => ()
            }
        }
    }
    
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}

