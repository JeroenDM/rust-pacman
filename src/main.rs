mod controller;
mod pacman;
mod tools;
mod view;

use std::convert::TryFrom;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, ResizeEvent};
use piston::window::{NoWindow, WindowSettings};
use piston::{Button, ButtonEvent, PressEvent, UpdateEvent};

use clap::{Parser, ValueEnum};

use crate::controller::Controller;
use crate::pacman::Pacman;
use crate::view::View;

use controller::Input;

fn button_to_input(button: Button) -> Input {
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

fn run_nogui(events: &mut Events, controller: &mut Controller) {
    let mut window = NoWindow::new(&WindowSettings::new("pacman-game", [750, 750]));

    while let Some(e) = events.next(&mut window) {
        if let Some(update) = e.update_args() {
            controller.update(update);
        }
    }
}

fn run(events: &mut Events, controller: &mut Controller) -> tools::Recording {
    let mut recording = tools::Recording::new();
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
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);
                view.draw(&controller, &c, g);
            })
        }
        if let Some(r) = e.resize_args() {
            view.resize(r.window_size[0], r.window_size[1]);
        }
        if let Some(update) = e.update_args() {
            controller.update(update);
            frame_count += 1;
            // recording.push((frame_count, Input::None.into()));
        }
        if let Some(button) = e.press_args() {
            let input = button_to_input(button);
            recording.push((frame_count, input.into()));
            if controller.input(input) {
                return recording;
            }
        }
    }
    return recording;
}

fn run_from_recoding(events: &mut Events, controller: &mut Controller, inputs: tools::Recording) {
    const GL_VERSION: OpenGL = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("pacman-game", [750, 750])
        .graphics_api(GL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(GL_VERSION);
    let mut view = View::new();

    let mut idx: usize = 0;

    let mut frame_count: u64 = 0;
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);
                view.draw(&controller, &c, g);
            })
        }
        if let Some(r) = e.resize_args() {
            view.resize(r.window_size[0], r.window_size[1]);
        }
        if let Some(update) = e.update_args() {
            controller.update(update);
            frame_count += 1;
            if idx >= inputs.len() {
                return;
            }
            if inputs[idx].0 == frame_count {
                match Input::try_from(inputs[idx].1) {
                    Ok(input) => {
                        controller.input(input);
                        idx += 1;
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
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
}

fn main() {
    let args = CliArgs::parse();

    println!(
        "run with arguments mode: {:?}, nogui: {:?}",
        args.mode, args.nogui
    );

    let should_render = !args.nogui;

    let mut controller = Controller::new(Pacman::new());
    let mut settings = EventSettings::new();
    // settings.bench_mode = true;
    settings.ups = 50;
    let mut events = Events::new(settings);

    if args.mode == AppMode::Replay {
        let recording = tools::read_recording_from_file("recording.txt").unwrap();
        run_from_recoding(&mut events, &mut controller, recording);
    } else if should_render {
        let recording = run(&mut events, &mut controller);
        tools::write_recording_to_file(&recording, "recording.txt").unwrap();
    } else {
        run_nogui(&mut events, &mut controller);
    }
}
