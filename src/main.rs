use core::panic;
use std::path::Path;
use std::str::FromStr;
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

#[derive(Debug)]
struct Cursor {
    line: u32,
    position: u32,
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

fn render_rectangle(
    x_axis: i32,
    y_axis: i32,
    width: u32,
    height: u32,
    canvas: &mut WindowCanvas,
    color: Color,
) {
    let rectangle = Rect::new(x_axis, y_axis, width, height);
    canvas.set_draw_color(color);

    canvas.fill_rect(rectangle).unwrap();
}

fn string_surface(font: &Font, text: &str) -> Surface<'static> {
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

    let line_surface = string_surface(font, line);

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
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window = get_window(&sdl_context, TITLE, WindowMode::Default);
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let sdl2_tff_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("src/JetBrainsMono-Regular.ttf");
    let font_size = 14u16;
    let font = sdl2_tff_context.load_font(font_path, font_size).unwrap();

    let text_renderer = get_text_renderer(&texture_creator, &font);

    let mut editor_text: Vec<String> = vec![
        String::from("Hey guys"),
        String::from(""),
        String::from("I am the second line"),
    ];
    let mut cursor = Cursor {
        line: 0,
        position: 0,
    };
    let mut cursor_shown = true;
    let mut repetition = 0u8;
    let fps = 60u8;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => set_cursor_line(&mut cursor, &editor_text, -1),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => set_cursor_line(&mut cursor, &editor_text, 1),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => set_cursor_position(&mut cursor, &editor_text, -1),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => set_cursor_position(&mut cursor, &editor_text, 1),
                Event::KeyDown {
                    keycode: Some(Keycode::End),
                    ..
                } => move_to_end(&mut cursor, &editor_text),
                Event::KeyDown {
                    keycode: Some(Keycode::Home),
                    ..
                } => move_to_start(&mut cursor),
                Event::TextInput { text, .. } => {
                    handle_text_input(text, &mut editor_text, &mut cursor);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => handle_backspace(&mut editor_text, &mut cursor),
                _ => {}
            }
        }

        render_canvas(&mut canvas, Color::from((20, 5, 0)));

        text_renderer(&editor_text, &mut canvas).unwrap();

        if repetition != fps / 2u8 {
            repetition = repetition + 1u8;
            if cursor_shown {
                render_cursor(&mut canvas, &cursor, &font);
            };
            if editor_text.len() < 15 {
                editor_text.push("Another line".to_string()); // Just for test
            }
        } else if cursor_shown {
            render_cursor(&mut canvas, &cursor, &font);
            repetition = 0u8;
            cursor_shown = false;
        } else {
            repetition = 0u8;
            cursor_shown = true;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps as u32));
    }
}

fn get_text_renderer<'b>(
    texture_creator: &'b TextureCreator<WindowContext>,
    font: &'b Font,
) -> impl for<'a> Fn(&Vec<String>, &mut WindowCanvas) -> Result<(), String> + 'b {
    let line_height = font.height();
    let text_renderer =
        move |text: &Vec<String>, canvas: &mut WindowCanvas| -> Result<(), String> {
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

fn move_to_end(cursor: &mut Cursor, text: &Vec<String>) {
    cursor.position = line_length(cursor.line as usize, text).unwrap() as u32;
}

fn move_to_start(cursor: &mut Cursor) {
    cursor.position = 0;
}

fn render_cursor(canvas: &mut WindowCanvas, cursor: &Cursor, font: &Font) {
    let line_height = font.height();
    let letter_width = string_surface(font, "_").width();

    let (cursor_x, cursor_y) =
        get_cursor_axis(line_height, letter_width, cursor.line, cursor.position);

    let color = Color::from((155u8, 230u8, 255u8));

    render_rectangle(
        cursor_x,
        cursor_y,
        1 as u32,
        line_height as u32,
        canvas,
        color,
    );
}

fn get_cursor_axis(line_height: i32, letter_width: u32, line: u32, position: u32) -> (i32, i32) {
    let cursor_x = (position as i32 * letter_width as i32) + PADDING;
    let cursor_y = (line as i32 * line_height as i32) + PADDING;

    (cursor_x, cursor_y)
}

fn set_cursor_position(cursor: &mut Cursor, text: &Vec<String>, offset: i8) {
    let (line, position) = {
        let new_position = cursor.position as i32 + offset as i32;
        let exceeds_line = new_position as usize > line_length(cursor.line as usize, text).unwrap();
        let next_line_exists = line_exists(text, cursor.line as usize + 1);
        let prev_line_exists = cursor.line > 0;

        if (!prev_line_exists && new_position < 0) || (!next_line_exists && exceeds_line) {
            (cursor.line, cursor.position)
        } else if new_position < 0 && prev_line_exists {
            let new_line = cursor.line - 1 as u32;
            (
                new_line,
                line_length(new_line as usize, text).unwrap() as u32,
            )
        } else if exceeds_line && next_line_exists {
            (cursor.line + 1, 0)
        } else {
            (cursor.line, new_position as u32)
        }
    };

    cursor.line = line;
    cursor.position = position
}

fn set_cursor_line(cursor: &mut Cursor, text: &Vec<String>, offset: i8) {
    let (line, position) = {
        let new_line = cursor.line as i32 + offset as i32;
        let new_line_length = line_length(new_line as usize, text).ok();

        if !new_line_length.is_some() {
            (cursor.line, cursor.position)
        } else if cursor.position as usize > new_line_length.unwrap() {
            (new_line as u32, new_line_length.unwrap() as u32)
        } else {
            (new_line as u32, cursor.position)
        }
    };

    cursor.position = position;
    cursor.line = line;
}

fn line_exists(text: &Vec<String>, line_index: usize) -> bool {
    match text.get(line_index) {
        None => false,
        Some(_) => true,
    }
}

fn line_length(line_index: usize, text: &Vec<String>) -> Result<usize, String> {
    let line = match text.get(line_index) {
        Some(line_text) => Ok(str::len(line_text)),
        None => Err(String::from_str(
            "Error in get_line_text_length: Line index does not return anything",
        )
        .unwrap()),
    };
    line
}

fn handle_text_input(string_input: String, text: &mut Vec<String>, cursor: &mut Cursor) {
    insert_str_in_text(string_input, text, cursor);
    cursor.position += 1;
}

fn insert_str_in_text(string_input: String, text: &mut Vec<String>, cursor: &Cursor) {
    let old_line = &text[cursor.line as usize];
    let mut new_line = old_line.to_string();
    new_line.insert_str(cursor.position as usize, &string_input);

    text[cursor.line as usize] = new_line;
}

fn handle_backspace(text: &mut Vec<String>, cursor: &mut Cursor) {
    if cursor.position > 0 {
        remove_char_under_cursor(text, cursor);
        cursor.position -= 1;
    }
}

fn remove_char_under_cursor(text: &mut Vec<String>, cursor: &mut Cursor) {
    let old_line = &text[cursor.line as usize];
    let mut new_line = old_line.to_string();
    new_line.remove(cursor.position as usize - 1);

    text[cursor.line as usize] = new_line;
}
