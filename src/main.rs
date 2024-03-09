use core::panic;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::Sdl;

extern crate sdl2;

enum WindowMode {
    // Resizable,
    // Fullscreen,
    Default,
}

const TITLE: &str = "Rusteed";
const PADDING: i32 = 12;

fn get_window(sdl_context: &Sdl, title: &str, window_mode: WindowMode) -> sdl2::video::Window {
    let video_subsystem = sdl_context.video().unwrap();

    match window_mode {
        // WindowMode::Resizable => video_subsystem
        //   .window(title, 800, 600)
        //   .resizable()
        //   .build()
        //   .unwrap(),
        // WindowMode::Fullscreen => video_subsystem
        //   .window(title, 800, 600)
        //   .fullscreen_desktop()
        //   .build()
        //   .unwrap(),
        WindowMode::Default => video_subsystem.window(title, 800, 600).build().unwrap(),
    }
}

fn get_string_surface(font: Font, text: &str) -> Surface<'static> {
    let partial_render = font.render(&text);

    let surface = match partial_render.solid(Color::WHITE) {
        Ok(surface) => surface,
        Err(error) => panic!("Following error solid partial rendering: {:?}", error),
    };

    surface
}

fn render_string(
    canvas: &mut WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    string_value: &str,
    font: Font,
    padding: i32,
) -> Result<(), String> {
    let string_surface = get_string_surface(font, string_value);

    let (width, height) = (string_surface.width(), string_surface.height());

    let text_texture = texture_creator
        .create_texture_from_surface(string_surface)
        .map_err(|err| err.to_string())?;

    let rendering_target = Rect::new(padding, padding, width, height);

    canvas.copy(&text_texture, None, rendering_target).unwrap();

    canvas.present();
    Ok(())
}

fn render_canvas(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window = get_window(&sdl_context, TITLE, WindowMode::Default);
    let mut canvas = window.into_canvas().build().unwrap();

    render_canvas(&mut canvas, Color::from((20, 5, 0)));

    let texture_creator = canvas.texture_creator();
    let sdl2_tff_context = sdl2::ttf::init().unwrap();
    let font = sdl2_tff_context
        .load_font("src/JetBrainsMono-Regular.ttf", 14)
        .unwrap();

    let mut text: Vec<&str> = vec![""];

    render_string(
        &mut canvas,
        texture_creator,
        "Initial default text",
        font,
        PADDING,
    )
    .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
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
