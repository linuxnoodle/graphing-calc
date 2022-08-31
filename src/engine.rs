extern crate sdl2;
extern crate egui;

pub mod calc;
pub mod converter;

use std::time::Instant;
use egui_sdl2_gl as egui_backend;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use calc::*;
//use converter::*;

// TODO: find some other, less janky mechanism to do this
pub struct EngineVars {
    ctx: sdl2::Sdl,
    cvs: sdl2::render::Canvas<sdl2::video::Window>,
    res: (i32, i32, i32), // x, y, unit size
    grid_offset: (i32, i32),
    mouse_button_pressed: bool,
    pub running_state: bool
}

pub fn init(size: (i32, i32, i32)) -> Box<EngineVars> {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let sdl_window = Box::new(video.window("", size.0 as u32, size.1 as u32)
        .position_centered()
        .build()
        .unwrap());
    
    let sdl_canvas = sdl_window.into_canvas().build().unwrap();
    Box::new(EngineVars {
        ctx: sdl_context,
        cvs: sdl_canvas,
        res: size,
        grid_offset: (0, 0),
        mouse_button_pressed: false,
        running_state: true
    })
}

pub fn update(evars: &mut Box<EngineVars>) {
    let mut event_pump = evars.ctx.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                evars.running_state = false;
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                evars.grid_offset.1 += 5;
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                evars.grid_offset.1 -= 5;
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                evars.grid_offset.0 += 5;
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                evars.grid_offset.0 -= 5;
            },
            Event::MouseButtonDown { mouse_btn: _, .. } => {
                evars.mouse_button_pressed = true;
            },
            Event::MouseButtonUp { mouse_btn: _, .. } => {
                evars.mouse_button_pressed = false;
            },
            Event::MouseWheel { y, .. } => {
                if y > 0 {
                    evars.res.2 += 1;
                } else {
                    evars.res.2 -= if evars.res.2 <= 2 { 0 } else { 1 };
                }
            },
            Event::MouseMotion { xrel, yrel, .. } => {
                if evars.mouse_button_pressed {
                    evars.grid_offset.0 += xrel;
                    evars.grid_offset.1 += yrel;
                }
            },
            _ => {}
        }
    }

}
    
pub fn render(evars: &mut Box<EngineVars>) {
    evars.cvs.set_draw_color(Color::RGB(0, 0, 0));
    evars.cvs.clear();

    evars.cvs.set_draw_color(Color::RGB(20, 120, 150));
    let (grid_off_xn, grid_off_yn) = (evars.grid_offset.0 % evars.res.2, evars.grid_offset.1 % evars.res.2);
    // vertical gridlines
    for i in 0..=(evars.res.0 / evars.res.2) {
        evars.cvs.draw_line((i * evars.res.2 + grid_off_xn, 0),
                           (i * evars.res.2 + grid_off_xn, evars.res.1)).unwrap();
    }
    // horizontal gridlines
    for i in 0..=(evars.res.1 / evars.res.2) {
        evars.cvs.draw_line((0, i * evars.res.2 + grid_off_yn),
                           (evars.res.0, i * evars.res.2 + grid_off_yn)).unwrap();
    }

    // overdraw axes because it's faster than an if statement
    evars.cvs.set_draw_color(Color::RGB(100, 200, 230));
    evars.cvs.draw_line(((evars.res.0 - evars.res.0 % evars.res.2) / 2 + evars.grid_offset.0, 0),
                       ((evars.res.0 - evars.res.0 % evars.res.2) / 2 + evars.grid_offset.0, evars.res.1)).unwrap();
    evars.cvs.draw_line((0, evars.res.1 / 2 + evars.grid_offset.1),
                       (evars.res.0, evars.res.1 / 2 + evars.grid_offset.1)).unwrap();

    // draw equation listed 
    // TODO: tie in with equation parser to run any equation
    evars.cvs.set_draw_color(Color::RGB(255, 255, 255));
    let delta_x = 0.01;
    let xb = evars.res.0 as f32 / 2.0;
    let yb = evars.res.1 as f32 / 2.0;

    let iterations = (evars.res.0 * (1.0 / delta_x) as i32)/ evars.res.2;
    for i in -iterations/2..iterations/2 {
        let x = i as f32 * delta_x - evars.grid_offset.0 as f32 / evars.res.2 as f32;
        let x_pixel = x * evars.res.2 as f32 + xb as f32 + evars.grid_offset.0 as f32;
        let y_pixel = -math_equation(x) * evars.res.2 as f32 + yb as f32 + evars.grid_offset.1 as f32;

        let xn_pixel = x * evars.res.2 as f32 + delta_x + xb as f32 + evars.grid_offset.0 as f32;
        let yn_pixel = -math_equation(x + delta_x) * evars.res.2 as f32 + yb as f32 + evars.grid_offset.1 as f32;
        evars.cvs.draw_line((x_pixel as i32, y_pixel as i32),
                           (xn_pixel as i32, yn_pixel as i32)).unwrap();
    }
    evars.cvs.present();
}

fn math_equation(x: f32) -> f32 {
    0.5 * x * x.sin()
}
