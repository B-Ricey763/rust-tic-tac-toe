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

fn display(board: &mut [Tile; 9]) {
    for tile in board {
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut board = [Tile::Empty; 9];
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.lock();

    stdout.execute(cursor::MoveTo(0, 0));
    write!(&mut stdout, "Hello World!")?;
    stdout.flush();

    loop {
        if let Event::Key(event) = event::read()? {
            let (r, c) = match event.code {
                KeyCode::Up => (1, 0),
                KeyCode::Down => (-1, 0),
                KeyCode::Left => (0, -1),
                KeyCode::Right => (0, 1),
                KeyCode::Esc => break,
                _ => (0, 0),
            };
        }
    }
    
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

