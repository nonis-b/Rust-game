extern crate sdl2; 

use crate::frame_stats::FrameStats;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture};
use sdl2::rect::{Rect};
use std::time::Duration;

mod frame_stats;

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        return Vec2{x: x, y: y};
    }
}

struct Sprite<'a> {
    pos: Vec2,
    size: Vec2,
    tex: &'a Texture<'a>,
}

fn update(obj: &mut Sprite, delta_seconds: f32) {
    obj.pos.x += 20.0 * delta_seconds;
    if obj.pos.x > 800.0 {
        obj.pos.x = 0.0;
    }
}

const DELTA_SECONDS: f32 = 1.0 / 60.0;

pub fn main() {
    let mut frame_stats = FrameStats::init();
    let mut objs = Vec::new();
    let png = "res/can.png";
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png).unwrap();
 
    objs.push(Sprite{
        pos: Vec2::new(10.0, 100.0),
        size: Vec2::new(50.0, 50.0),
        tex: &texture
    });

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        let frame_start = timer_subsystem.ticks();
        frame_stats.tick_and_print(frame_start);

        for o in &mut objs {
            update(o, DELTA_SECONDS);
        }

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for o in &objs {
            canvas.copy(o.tex, None, Rect::new(
                o.pos.x as i32, o.pos.y as i32, 
                o.size.x as u32, o.size.y as u32)).unwrap();
        }

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        let delta_ms = (DELTA_SECONDS * 1000.0) as i64;
        let frame_end = timer_subsystem.ticks();
        let ms_left = delta_ms - (frame_end - frame_start) as i64;
        if ms_left > 0 {
            let sleep_ms = if ms_left > delta_ms { delta_ms } else { ms_left };
            let sleep_ns = sleep_ms * 1_000_000;
            ::std::thread::sleep(Duration::new(0, sleep_ns as u32));
        }
    }
}