extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::Gl;
use sdl2_game_window::WindowSDL2;

fn main() {
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        piston::WindowSettings::default());
    let event_settings = piston::EventSettings {
        updates_per_second:    120,
        max_frames_per_second: 60
    };
    let ref mut gl = Gl::new(opengl);
    for e in piston::EventIterator::new(&mut window, &event_settings) {};
}
