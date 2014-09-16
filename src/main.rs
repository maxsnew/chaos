extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_game_window;

use num::complex::Complex;
use opengl_graphics::{Gl, Texture};
use piston::graphics::{AddColor, AddImage, Context, Draw};
use piston::image::Pixel;
use piston::{Render, image};
use piston::image::GenericImage;
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
    let event_settings = piston::EventSettings {
        updates_per_second:    120,
        max_frames_per_second: 60
    };
    let ref mut gl = Gl::new(opengl);
    let max_iterations = 256u16;
    let imgx = 800;
    let imgy = 800;
    let scalex = 4.0 / imgx as f32;
    let scaley = 4.0 / imgy as f32;
    //Create a new ImgBuf with width: imgx and height: imgy
    let mut imbuf: image::ImageBuf<image::Rgba<u8>> = image::ImageBuf::new(imgx, imgy);
    for y in range(0, imgy) {
        let cy = y as f32 * scaley - 2.0;
        for x in range(0, imgx) {
            let cx = x as f32 * scalex - 2.0;
            let mut z = Complex::new(cx, cy);
            let c = Complex::new(-0.4, 0.6);
            let mut i = 0;
            for t in range(0, max_iterations) {
                if z.norm() > 2.0 {
                    break
                }
                z = z * z + c;
                i = t;
            }
            // Create an 8bit pixel of type Luma and value i
            let pixel = image::Luma(i as u8).to_rgba();
            // Put a pixel in the image at coordinates x and y
            imbuf.put_pixel(x, y, pixel);
        }
    }
    let mut texture = Texture::from_image(&imbuf);
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
