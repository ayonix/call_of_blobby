extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::input::Button;
use piston::input::keyboard::Key;
use piston::event::*;
use graphics::{
    Context,
    ellipse,
    RelativeTransform
};
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player1: Player,
}

struct Player {
    x: f64,
    y: f64,
    color: [f32; 4],
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

impl App {
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);

        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64)
            .trans(-25.0, -25.0);

        let circle = ellipse::circle(self.player1.x, self.player1.y, 50.0);
        let color1 = self.player1.color;

        self.gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
            // Clear the screen.
            graphics::clear(BLACK, gl);
            graphics::ellipse(color1, circle, center_context.transform, gl);
        });
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
    }
}

fn main() {
    // Create an SDL window.
    let window = Window::new(
        OpenGL::_3_2,
        WindowSettings::default()
    );
    let window = RefCell::new(window);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(OpenGL::_3_2),
        player1: Player {x: 0.0, y: 0.0, color: [0.5, 0.5, 0.5, 1.0]},
    };

    for e in events(&window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => {
                    app.player1.x -= 1.0;
                },
                Key::Right => {
                    app.player1.x += 1.0;
                },
                Key::Up => {
                    app.player1.y -= 1.0;
                },
                Key::Down => {
                    app.player1.y += 1.0;
                }
                _ => {
                    println!("Pressed {:?}", key);
                }
            }

        }

        if let Some(r) = e.render_args() {
            app.render(&mut window.borrow_mut(), &r);
        }

        if let Some(u) = e.update_args() {
            app.update(&mut window.borrow_mut(), &u);
        }
    }
}
