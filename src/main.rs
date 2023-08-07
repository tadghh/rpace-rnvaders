use std::{error::Error, io, time::{Duration, Instant}, sync::mpsc, thread};
use crossterm::{terminal::{EnterAlternateScreen, self, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}};
use rpace_rnvaders::{frame::{self, new_frame, Drawable}, render, player::Player, invaders::Invaders};
use rusty_audio::Audio;
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("win", "win.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");

    //Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        //Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut  curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_e) = event::read()? {
                match key_e.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot(){
                            audio.play("pew")
                        }
                    },
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }
          //updates
        player.update(delta);
        if invaders.update(delta){
            audio.play("move");
        }
        if player.detect_hits(&mut invaders){
            audio.play("explode");
        }
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables{
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        //Win/lose
        if invaders.all_killed(){
            break 'gameloop;
        }
        if invaders.reached_bottom(){
            break 'gameloop;
        }
    }



    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    //Wrap it up :)
    terminal::disable_raw_mode()?;
    audio.wait();
    Ok(())
}
