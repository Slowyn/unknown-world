extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    planet_rotation: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 0.8, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.95];

        let radius = 220.0;

        let planet = ellipse::circle(0.0, 0.0, 30.0);

        let center = ellipse::circle(0.0, 0.0, 80.0);
        let point = ellipse::circle(0.0, 0.0, 2.0);

        let satellite = ellipse::circle(0.0, 0.0, 10.0);

        let rotation = self.rotation;
        let planet_rotation = self.planet_rotation;
        let (center_x, center_y) = (args.width / 2.0, args.height / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let (planet_x, planet_y) = (
                radius * planet_rotation.cos() + center_x - 15.0,
                radius * planet_rotation.sin() + center_y - 15.0,
            );

            let (satellite_x, satellite_y) = (
                60.0 * planet_rotation.sin() + planet_x,
                60.0 * planet_rotation.cos() + planet_y,
            );

            let transform = c.transform
                .trans(planet_x, planet_y)
                .rot_rad(rotation);

            let center_transform = c.transform
                .trans(center_x, center_y)
                .trans(-1.0, -1.0);

            let point_transform = c.transform
                .trans(planet_x, planet_y)
                .trans(-1.0, -1.0);

            let satellite_transform = c.transform
                .trans(satellite_x, satellite_y)
                .rot_rad(rotation);

            let line_transform = c.transform.trans(-0.5, -0.5);

            ellipse(RED, planet, transform, gl);
            ellipse(RED, satellite, satellite_transform, gl);
            ellipse(BLACK, point, point_transform, gl);
            ellipse(BLACK, point, center_transform, gl);
            ellipse(GREEN, center, center_transform, gl);
//            line(
//                GREEN,
//                1.0,
//                [center_x, center_y, planet_x, planet_y],
//                line_transform,
//                gl,
//            );
//            line(
//                GREEN,
//                1.0,
//                [center_x, center_y, satellite_x, satellite_y],
//                line_transform,
//                gl,
//            );
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
        self.planet_rotation += 1.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Unknown World",
        [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        planet_rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
