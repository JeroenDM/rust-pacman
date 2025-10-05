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
use crate::sim::{FileLoader, RandGen, Simulator};
use crate::view::View;

const GL_VERSION: OpenGL = OpenGL::V4_5;
/// Number of sequence the main loop should tick the game in gui mode.
const UPDATE_HZ: u64 = 6;

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

fn try_parse_recording(recording: sim::Recording) -> Result<Vec<(u64, game::Input)>, String> {
    // None-empty.
    let last_input = recording.last().ok_or("Empty recording.".to_string())?;

    // Finite.
    if last_input.1 != 'q' {
        return Err("This recording will never quite.".to_string());
    }

    // Valid data.
    let mut inputs = Vec::<(u64, game::Input)>::new();
    inputs.reserve(recording.len());
    for (count, char) in &recording.clone() {
        let input = game::Input::try_from(*char)?;
        inputs.push((*count, input));
    }

    Ok(inputs)
}

struct Buffer<T: Clone + Copy> {
    x: Option<T>,
}

impl<T: Clone + Copy> Buffer<T> {
    fn new() -> Self {
        Self { x: None }
    }

    fn is_empty(&self) -> bool {
        self.x.is_none()
    }

    fn push(&mut self, most_recent_input: T) {
        self.x = Some(most_recent_input);
    }

    fn pop(&mut self) -> Option<T> {
        let input = self.x;
        self.x = None;
        input
    }
}

#[derive(Debug, Default)]
struct Sim1 {
    x: usize,
}

impl RandGen for Sim1 {
    fn rand(&mut self) -> usize {
        self.x += 1;
        self.x
    }
}

impl FileLoader for Sim1 {
    fn load_file(&mut self, _filename: &str) -> String {
        let map = r#"############################
            #................X.........#
            #..........................#
            #..........................#
            #..........................#
            #..............X.X.........#
            #..........................#
            #..........................#
            #..........................#
            ############################"#;
        return map.to_string();
    }
}

impl Simulator for Sim1 {}

fn maybe_render<RG: Simulator>(
    e: &piston::Event,
    game: &Game<RG>,
    gl: &mut GlGraphics,
    view: &mut View,
) {
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

fn run<RG: Simulator>(events: &mut Events, game: &mut Game<RG>) -> sim::Recording {
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
    let mut input_source = Buffer::<game::Input>::new();
    while let Some(e) = events.next(&mut window) {
        // Input
        if let Some(button) = e.press_args() {
            println!("[{}]-- input --", frame_count);
            let input = button_to_input(button);
            // This will overwrite the previous input.
            // The previous input is potentially never used if there was no update call
            // below in the current frame.
            input_source.push(input);
        }

        // Update
        if e.update_args().is_some() {
            if let Some(input) = input_source.pop() {
                recording.push((frame_count, input.into()));
                if game.input(input) {
                    return recording;
                }
            }
            println!("[{}]-- update --", frame_count);
            game.update();
            frame_count += 1;
        }
        maybe_render(&e, &game, &mut gl, &mut view);
    }
    return recording;
}

fn run_from_recording_nogui<RG: Simulator>(
    game: &mut Game<RG>,
    recording: sim::Recording,
) -> Result<(), String> {
    let inputs = try_parse_recording(recording)?;
    let max_frame_count = inputs.last().unwrap().0;

    let mut idx = 0;
    for frame_count in 0..max_frame_count {
        // I think this cannot go out of bounds because of the validation for 'q'
        // in `try_parse_recording`.
        assert!(idx < inputs.len());

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

fn run_from_recoding<RG: Simulator>(
    events: &mut Events,
    game: &mut Game<RG>,
    recording: sim::Recording,
) -> Result<(), String> {
    let mut window: Window = WindowSettings::new("pacman-game", [750, 750])
        .graphics_api(GL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(GL_VERSION);
    let mut view = View::new();

    let inputs = try_parse_recording(recording)?;
    let mut idx_input: usize = 0;

    let mut frame_count: u64 = 0;
    let mut input_source = Buffer::<game::Input>::new();

    while let Some(e) = events.next(&mut window) {
        // I think this cannot go out of bounds because of the validation for 'q'
        // in `try_parse_recording`.
        assert!(idx_input < inputs.len());
        // For the same reson we cannot run past the largest frame count.
        assert!(frame_count < inputs.last().unwrap().0);

        if input_source.is_empty() && inputs[idx_input].0 == frame_count {
            input_source.push(inputs[idx_input].1);
            idx_input += 1;
        }

        if let Some(_) = e.update_args() {
            if game.input(input_source.pop().unwrap_or(game::Input::None)) {
                return Ok(());
            }
            game.update();
            frame_count += 1;
        }
        maybe_render(&e, &game, &mut gl, &mut view);
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

    #[arg(long, default_value = "recording.game.txt")]
    recording_filepath: String,
}

fn main() {
    let args = CliArgs::parse();

    println!(
        "run with arguments mode: {:?}, nogui: {:?}",
        args.mode, args.nogui
    );

    let should_render = !args.nogui;

    let mut game = Game::<Sim1>::new();
    let mut settings = EventSettings::new();
    // settings.bench_mode = true;
    settings.ups = (UPDATE_HZ as f64 * args.playback_speed) as u64;
    let mut events = Events::new(settings);

    if args.mode == AppMode::Replay && should_render {
        let recording = sim::read_recording_from_file(&args.recording_filepath).unwrap();
        if let Err(e) = run_from_recoding(&mut events, &mut game, recording) {
            eprintln!("ERROR: {e}");
        }
    } else if args.mode == AppMode::Replay && !should_render {
        let recording = sim::read_recording_from_file(&args.recording_filepath).unwrap();
        if let Err(e) = run_from_recording_nogui(&mut game, recording) {
            eprintln!("ERROR: {e}");
        }
    } else if args.mode == AppMode::Record && should_render {
        let recording = run(&mut events, &mut game);
        sim::write_recording_to_file(&recording, &args.recording_filepath).unwrap();
    } else {
        panic!("Invalid options");
    }

    println!("Recording finished. Score: {}", game.get_stats().score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example_recording() {
        // We can use snapshot testing here!
        let mut game = Game::<Sim1>::new();
        let recording = sim::read_recording_from_file("test_game_file.txt").unwrap();
        assert_eq!(run_from_recording_nogui(&mut game, recording), Ok(()));
    }
}
