extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use graphics::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::{Button, Key};

const gravity: f64 = 2.0;
const groundHeight: f64 = 100.0;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    players: Vec<Player>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let ref players = self.players;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for p in players {
                p.render(&c, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for p in &mut self.players {
            p.update()
        }
    }
}

struct Player {
    x: f64,
    y: f64,
    radius: f64,
    color: [f32; 4],
    velocity: f64,
    jumping: bool
}

impl Player {
    pub fn new(x: f64, y: f64, radius: f64, color: [f32; 4]) -> Player {
        Player{x: x, y: y, radius: radius, color: color, velocity: 0.0, jumping: false}
    }

    fn render(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        rectangle(self.color, rectangle::square(self.x, self.y, self.radius), c.transform, gl)
    }

    fn jump(&mut self) {
        if !self.jumping {
            self.velocity = -20.0; 
            self.y += self.velocity;
            self.jumping = true;
        }
    }

    fn update(&mut self) {

        if self.jumping {
            self.y += self.velocity;
            self.velocity += gravity;

            if self.y >= groundHeight {
                self.y = groundHeight;
                self.jumping = false;
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Call of Blobby",
            [200, 200]
        )
        .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        players: vec![Player::new(0.0, groundHeight, 50.0, [1.0, 1.0, 0.0, 1.0]),
                    Player::new(20.0, groundHeight, 50.0, [0.0, 0.5, 0.5, 1.0])]
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::D {
                app.players[0].x += 20.0;
            }
            if key == Key::A {
                app.players[0].x -= 20.0;
            }
            if key == Key::W {
                app.players[0].jump();
            }
            if key == Key::L {
                app.players[1].x += 20.0;
            }
            if key == Key::J {
                app.players[1].x -= 20.0;
            }
            if key == Key::I {
                app.players[1].jump();
            }
        }
    }
}