mod game;
mod sim;
mod view;

use std::convert::TryFrom;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, ResizeEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent, UpdateEvent};

use clap::{Parser, ValueEnum};

use crate::game::Game;
use crate::view::View;

const GL_VERSION: OpenGL = OpenGL::V4_5;
/// Number of sequence the main loop should tick the game in gui mode.
const UPDATE_HZ: u64 = 4;

fn button_to_input(button: Button) -> game::Input {
    use game::Input;
    use piston::input::keyboard::Key;
    use piston::input::Button::Keyboard;

    match button {
        Keyboard(Key::Up) | Keyboard(Key::I) => Input::Up,
        Keyboard(Key::Down) | Keyboard(Key::K) => Input::Down,
        Keyboard(Key::Left) | Keyboard(Key::J) => Input::Left,
        Keyboard(Key::Right) | Keyboard(Key::L) => Input::Right,
        Keyboard(Key::Q) => Input::Quit,
        Keyboard(Key::P) => Input::Pause,
        _ => Input::None,
    }
}

fn run(events: &mut Events, game: &mut Game) -> sim::Recording {
    let mut recording = sim::Recording::new();
    const GL_VERSION: OpenGL = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("pacman-game", [750, 750])
        .graphics_api(GL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(GL_VERSION);
    let mut view = View::new();

    let mut frame_count: u64 = 0;
    while let Some(e) = events.next(&mut window) {
        // Input
        if let Some(button) = e.press_args() {
            let input = button_to_input(button);
            recording.push((frame_count, input.into()));
            if game.input(input) {
                return recording;
            }
        }

        // Render
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);
                view.draw(&game, &c, g);
            })
        }
        if let Some(r) = e.resize_args() {
            view.resize(r.window_size[0], r.window_size[1]);
        }

        // Update
        if e.update_args().is_some() {
            game.update();
            frame_count += 1;
        }
    }
    return recording;
}

fn run_from_recording_nogui(game: &mut Game, recording: sim::Recording) -> Result<(), String> {
    // Input Validation
    let last_input = recording.last().ok_or("Empty recording.".to_string())?;
    if last_input.1 != 'q' {
        return Err("This recording will never quite.".to_string());
    }

    let mut inputs = Vec::<(u64, game::Input)>::new();
    inputs.reserve(recording.len());
    for (count, char) in &recording {
        let input = game::Input::try_from(*char)?;
        inputs.push((*count, input));
    }

    let mut idx = 0;
    for frame_count in 0..last_input.0 {
        // I think this cannot go out of bounds because of the validation for 'q' above.
        if inputs[idx].0 == frame_count {
            if game.input(inputs[idx].1) {
                return Ok(());
            }
            idx += 1;
        }
        game.update();
    }

    Ok(())
}

fn run_from_recoding(
    events: &mut Events,
    game: &mut Game,
    recording: sim::Recording,
) -> Result<(), String> {
    let mut window: Window = WindowSettings::new("pacman-game", [750, 750])
        .graphics_api(GL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Input Validation
    let last_input = recording.last().ok_or("Empty recording.".to_string())?;
    if last_input.1 != 'q' {
        return Err("This recording will never quite.".to_string());
    }

    let mut inputs = Vec::<(u64, game::Input)>::new();
    inputs.reserve(recording.len());
    for (count, char) in &recording {
        let input = game::Input::try_from(*char)?;
        inputs.push((*count, input));
    }

    let mut gl = GlGraphics::new(GL_VERSION);
    let mut view = View::new();
    let mut idx: usize = 0;

    let mut frame_count: u64 = 0;
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.update_args() {
            // I think this cannot go out of bounds because of the validation for 'q' above.
            if inputs[idx].0 == frame_count {
                if game.input(inputs[idx].1) {
                    return Ok(());
                }
                idx += 1;
            }
            // Update
            game.update();
            frame_count += 1;
        }
        // Render
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);
                view.draw(&game, &c, g);
            })
        }
        if let Some(r) = e.resize_args() {
            view.resize(r.window_size[0], r.window_size[1]);
        }
    }

    Ok(())
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
enum AppMode {
    Record,
    Replay,
}

#[derive(Parser)]
#[command(name = "pacman-sim")]
#[command(about = "A deterministic pacman simulator.")]
struct CliArgs {
    #[arg(long, value_enum, default_value = "record")]
    mode: AppMode,

    #[arg(long)]
    nogui: bool,

    #[arg(long, default_value = "1.0")]
    playback_speed: f64,
}

fn main() {
    let args = CliArgs::parse();

    println!(
        "run with arguments mode: {:?}, nogui: {:?}",
        args.mode, args.nogui
    );

    let should_render = !args.nogui;

    let mut game = Game::new();
    let mut settings = EventSettings::new();
    // settings.bench_mode = true;
    settings.ups = (UPDATE_HZ as f64 * args.playback_speed) as u64;
    let mut events = Events::new(settings);

    if args.mode == AppMode::Replay && should_render {
        let recording = sim::read_recording_from_file("recording.game.txt").unwrap();
        if let Err(e) = run_from_recoding(&mut events, &mut game, recording) {
            eprintln!("ERROR: {e}");
        }
    } else if args.mode == AppMode::Replay && !should_render {
        let recording = sim::read_recording_from_file("recording.game.txt").unwrap();
        if let Err(e) = run_from_recording_nogui(&mut game, recording) {
            eprintln!("ERROR: {e}");
        }
    } else if args.mode == AppMode::Record && should_render {
        let recording = run(&mut events, &mut game);
        sim::write_recording_to_file(&recording, "recording.game.txt").unwrap();
    } else {
        panic!("Invalid options");
    }

    println!("Recording finished. Score: {}", game.get_stats().score);
}
