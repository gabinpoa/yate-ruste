use core::panic;
use std::path::Path;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

extern crate sdl2;

fn get_text_surface(text: String) -> Surface<'static> {
    let sdl2_tff_context = match sdl2::ttf::init() {
        Ok(context) => context,
        Err(error) => panic!("Following error initializing the ttf context: {:?}", error),
    };

    let font = match sdl2_tff_context.load_font(Path::new("src/JetBrainsMono-Regular.ttf"), 18) {
        Ok(font) => font,
        Err(error) => panic!("Following error loading font : {:?}", error),
    };

    let partial_render = font.render(&text);

    let surface = match partial_render.solid(Color::WHITE) {
        Ok(surface) => surface,
        Err(error) => panic!("Following error solid partial rendering: {:?}", error),
    };

    surface
}

fn render_text(
    canvas: &mut WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    text: String,
    padding: i32,
) -> Result<(), String> {
    let text_surface = get_text_surface(text);
    let (width, height) = (text_surface.width(), text_surface.height());
    let text_texture = texture_creator
        .create_texture_from_surface(text_surface)
        .map_err(|err| err.to_string())?;

    let rendering_target = Rect::new(padding, padding, width, height);

    canvas.set_draw_color(Color::WHITE);
    let _ = canvas.draw_rect(rendering_target);

    let _ = canvas.copy(&text_texture, None, rendering_target)?;
    canvas.present();
    Ok(())
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("My window demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::from((20, 5, 0)));
    canvas.clear();
    canvas.present();

    let _ = render_text(
        &mut canvas,
        texture_creator,
        String::from("Hello, world"),
        12,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
