use core::panic;
use std::path::Path;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};
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

fn get_string_surface(font: &Font, text: &str) -> Surface<'static> {
    let partial_render = font.render(&text);

    let surface = match partial_render.solid(Color::WHITE) {
        Ok(surface) => surface,
        Err(error) => panic!("Following error solid partial rendering: {:?}", error),
    };

    surface
}

fn render_line(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    line: &str,
    font: &Font,
    x_axis: i32,
    y_axis: i32,
) -> Result<(), String> {
    if line.len() == 0usize {
        return Ok(());
    }

    let line_surface = get_string_surface(font, line);

    let (width, height) = (line_surface.width(), line_surface.height());

    let line_texture = texture_creator
        .create_texture_from_surface(line_surface)
        .map_err(|err| err.to_string())?;

    let rendering_target = Rect::new(x_axis, y_axis, width, height);

    canvas.copy(&line_texture, None, rendering_target).unwrap();
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
    let texture_creator = canvas.texture_creator();

    render_canvas(&mut canvas, Color::from((20, 5, 0)));

    let sdl2_tff_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("src/JetBrainsMono-Regular.ttf");
    let font_size = 14u16;
    let text_renderer =
        get_text_renderer(&texture_creator, &sdl2_tff_context, font_path, font_size);

    let text: Vec<&str> = vec!["Hey guys", "", "I am the second line"];

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

        text_renderer(&text, &mut canvas).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 24));
    }
}

fn get_text_renderer<'b>(
    texture_creator: &'b TextureCreator<WindowContext>,
    sdl2_tff_context: &'b Sdl2TtfContext,
    font_path: &Path,
    font_size: u16,
) -> impl for<'a> Fn(&Vec<&'a str>, &mut WindowCanvas) -> Result<(), String> + 'b {
    let font = sdl2_tff_context.load_font(font_path, font_size).unwrap();
    let line_height = font.height();

    let text_renderer = move |text: &Vec<&str>, canvas: &mut WindowCanvas| -> Result<(), String> {
        for (i, line) in text.iter().enumerate() {
            render_line(
                canvas,
                &texture_creator,
                line,
                &font,
                PADDING,
                PADDING + (line_height * i as i32),
            )?;
        }

        Ok(())
    };

    text_renderer
}
