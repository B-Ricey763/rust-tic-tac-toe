use std::io::{self, Write, stdout};
use std::{
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

    fn display(&self) {
        let mut stdout = stdout();
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
        writeln!(&mut stdout, "{}", display_str);
    }
    fn set_tile(&mut self, tile: Tile, row: u16, col: u16) {
        let index = (row * 3 + col) as usize;
        self.array[index] = tile;
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut board = Board::new();
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.lock();

    stdout.execute(cursor::MoveTo(0, 0));

    let mut row = 0;
    let mut col = 0;
    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        stdout.execute(cursor::SavePosition);
        stdout.execute(cursor::MoveTo(0,0));
        board.display();
        stdout.flush()?;
        stdout.execute(cursor::RestorePosition);
        if let Event::Key(event) = event::read()? {
            let (r, c) = match event.code {
                KeyCode::Up => (0, -1),
                KeyCode::Down => (0, 1),
                KeyCode::Left => (-1, 0),
                KeyCode::Right => (1, 0),
                KeyCode::Esc => break,
                _ => (0, 0),
            };

            if event.code == KeyCode::Enter {
                // We have to switch these for some reason
                board.set_tile(Tile::X, col, row);
            }

            let (cursor_r, cursor_c) = cursor::position()?;
            row = (cursor_r as i8 + r) as u16;
            col = (cursor_c as i8 + c) as u16;
            stdout.execute(cursor::MoveTo(row, col));
        }
    }
    
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

