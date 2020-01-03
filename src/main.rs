extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate lib;

use piston::event_loop::*;
use piston::input::*;

use lib::App;
use lib::config::GraphicsConfig;

fn main() {
    // let opengl = OpenGL::V3_2;

    // let mut window: Window = WindowSettings::new(
    //         "spinning-square",
    //         [200, 200]
    //     )
    //     .graphics_api(opengl)
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();

    let mut app = App::new(GraphicsConfig::new("Square", 960.0, 768.0));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(i) = e.press_args() {
            app.input(&i, true);
        }

        if let Some(i) = e.release_args() {
            app.input(&i, false);
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }


}