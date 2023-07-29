use std::{error::Error, io, time::Duration};
use crossterm::{terminal::{EnterAlternateScreen, self, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}};
use rusty_audio::Audio;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut audio = Audio::new();
    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("win", "./sounds/win.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("pew", "./sounds/pew.wav");
    audio.add("startup", "./sounds/startup.wav");

    //Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;


    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_e) = event::read()? {
                match key_e.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    },
                    _ => {

                    }
                }
            }
        }
    }

    //Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    //Wrap it up :)
    terminal::disable_raw_mode()?;
    audio.wait();
    Ok(())
}
