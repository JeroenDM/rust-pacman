mod controller;
mod pacman;
mod view;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, ResizeEvent};
use piston::window::{NoWindow, WindowSettings};
use piston::{Button, ButtonEvent, PressEvent, UpdateEvent};

use crate::controller::Controller;
use crate::pacman::Pacman;
use crate::view::View;

fn run_nogui(events: &mut Events, controller: &mut Controller) {
    let mut window = NoWindow::new(&WindowSettings::new("pacman-game", [750, 750]));

    while let Some(e) = events.next(&mut window) {
        if let Some(update) = e.update_args() {
            controller.update(update);
        }
    }
}

fn run(events: &mut Events, controller: &mut Controller) -> Vec<(u64, Button)> {
    let mut recording = Vec::new();
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
        }
        if let Some(button) = e.press_args() {
            recording.push((frame_count, button));
            if controller.input(button) {
                return recording;
            }
        }
    }
    return recording;
}

fn run_from_recoding(events: &mut Events, controller: &mut Controller, inputs: Vec<(u64, Button)>) {
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
                controller.input(inputs[idx].1);
                idx += 1;
            }
        }
        // if let Some(button) = e.press_args() {
        //     use piston::input::keyboard::Key;
        //     if button == Button::Keyboard(Key::Q) {
        //         return;
        //     }
        // }
    }
}

fn main() {
    let should_render = true;

    let mut controller = Controller::new(Pacman::new());
    let mut settings = EventSettings::new();
    // settings.bench_mode = true;
    settings.ups = 50;
    let mut events = Events::new(settings);

    if should_render {
        let recording = run(&mut events, &mut controller);
        println!("{:?}", recording);

        // use piston::input::keyboard::Key;
        // let inputs = vec![Button::Keyboard(Key::Up); 100];
        run_from_recoding(&mut events, &mut controller, recording);
    } else {
        run_nogui(&mut events, &mut controller);
    }
}
