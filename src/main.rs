extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_game_window;

use opengl_graphics::{Gl, Texture};
use piston::graphics::{AddColor, AddImage, Context, Draw};
use piston::{Render, image};
use sdl2_game_window::WindowSDL2;

fn main() {
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let (w, h) = (300u32, 300u32);
    let window_settings = piston::WindowSettings {
        title: "Chaos".to_string(),
        size: [w, h],
        ..piston::WindowSettings::default()
    };
    let mut window = WindowSDL2::new(opengl, window_settings);
    let mut image = image::ImageBuf::new(w, h);
    let mut texture = Texture::from_image(&image);
    let event_settings = piston::EventSettings {
        updates_per_second:    120,
        max_frames_per_second: 60
    };
    let ref mut gl = Gl::new(opengl);
    for e in piston::EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                c.image(&texture).draw(gl);
            }
            _ => {}
        };
    };
}
