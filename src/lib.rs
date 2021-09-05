use std::{error::Error, env, thread, time::Duration};
use std::io::{self, Write, stdout};
use crossterm::{
    ExecutableCommand,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
    event::{self, Event, KeyCode}
};


/* 
#[derive(Copy, Clone)]
enum Tile {
    // Enum variants can also have values
    Empty = 0,
    Circle = 1,
    Cross = -1,
}

let tiles = [Tile::Empty, Tile::Empty, Tile::Cross];
tiles.iter().map(|tile| *tile as i32).sum::<i32>()
*/
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

type Board = [[Tile; MAX_LEN]; MAX_LEN];

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut board: Board = [[Tile::Empty; 3]; 3];
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.lock();

    stdout.execute(cursor::MoveTo(5, 5));
    write!(&mut stdout, "Hello World!")?;
    stdout.flush();

    loop {
        if let Event::Key(event) = event::read()? {
            match event.code 
                KeyCode::Up => println!("UP"),
                KeyCode::Esc => break,
                _ => (),
            }
        }
    }
    
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

