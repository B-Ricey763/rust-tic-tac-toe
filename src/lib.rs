use std::io::{self, Write, stdout};
use std::{
    self,
    fmt,
    error::Error, 
    time::Duration,
    env, 
    thread 
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    cursor
};

const MAX_LEN: usize = 3;

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
            let row = i / 3;

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

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut board = Board::new();
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.flush()?;
    stdout.lock();

    stdout.execute(cursor::MoveTo(0, 0));

    let mut row = 0;
    let mut col = 0;
    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        stdout.execute(cursor::SavePosition);
        stdout.execute(cursor::MoveTo(0,0));
        write!(&mut stdout, "{}", board);
        stdout.flush()?;
        stdout.execute(cursor::RestorePosition);
        if let Event::Key(event) = event::read()? {
            let result = match event.code {
                KeyCode::Char(c) => {
                    if Some(c) = c.to_digit(10) && c >= 0 && c < 9 {
                        Some(c)
                    } else {
                        None
                    }
                }
            };
            // TODO: Add a way to escape, using esc or q
            if Some(index) = result {
                // We have to switch these for some reason
                board.set_tile(Tile::X, index);
            }

            let (cursor_r, cursor_c) = cursor::position()?;
            row = (cursor_r as i8 + r) as u16;
            col = (cursor_c as i8 + c) as u16;
            stdout.execute(cursor::MoveTo(row, col));
        }
    }
    
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush();
    Ok(())
}

