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

pub struct EngineVars {
    ctx: sdl2::Sdl,
    cvs: sdl2::render::Canvas<sdl2::video::Window>,
    winr: Box<sdl2::video::Window>,
    ectx: egui::CtxRef,
    estate: egui_backend::EguiStateHandler,
    res: (i32, i32, i32), // x, y, unit size
    pub start_time: Instant,
    pub running_state: bool
}

pub fn init(size: (i32, i32, i32)) -> Box<EngineVars> {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let sdl_window = Box::new(video.window("", size.0 as u32, size.1 as u32)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap());
    
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);

    // egui init
    let shader_ver = egui_backend::ShaderVersion::Adaptive;
    let (_painter, egui_state) = 
        egui_backend::with_sdl2(&sdl_window, shader_ver, egui_backend::DpiScaling::Custom(2.0));
    let egui_ctx = egui::CtxRef::default();

    let sdl_canvas = &sdl_window.into_canvas().build().unwrap();
    Box::new(EngineVars {
        ctx: sdl_context,
        cvs: sdl_canvas,
        ectx: egui_ctx,
        estate: egui_state,
        winr: sdl_window,
        res: size,
        start_time: Instant::now(),
        running_state: true
    })
}

pub fn update(vars: &mut Box<EngineVars>) {
    let mut event_pump = vars.ctx.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => {
                vars.running_state = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                vars.running_state = false;
            },
            _ => {}
        }
    }

}
    
pub fn render(vars: &mut Box<EngineVars>) {
    // egui rendering
    let mut _egui_output = vars.ectx.begin_frame(vars.estate.input.take());
    egui::CentralPanel::default().show(&vars.ectx, |ui| {
        ui.heading("Hello World!");
    });
    let (_egui_output, _paint_cmds) = vars.ectx.end_frame();

    for i in 0..=(vars.res.0 / vars.res.2) {
        vars.cvs.draw_line((i * vars.res.2, 0), (i * vars.res.2, vars.res.1)).unwrap();
    }
    for i in 0..=(vars.res.1 / vars.res.2) {
        vars.cvs.draw_line((0, i * vars.res.2), (vars.res.0, i * vars.res.2)).unwrap();
    }

    // overdraw axes because it's faster than an if statement
    vars.cvs.set_draw_color(Color::RGB(100, 200, 230));
    vars.cvs.draw_line((0, vars.res.1 / 2), (vars.res.0, vars.res.1 / 2)).unwrap();
    vars.cvs.draw_line((vars.res.0 / 2, 0), (vars.res.0 / 2, vars.res.1)).unwrap();

    // draw equation (x^2 in this case)
    vars.cvs.set_draw_color(Color::RGB(255, 255, 255));
    let delta_x = 0.1;
    let xb = vars.res.0 as f32 / 2.0;
    let yb = vars.res.1 as f32 / 2.0;

    // negative x
    for i in 0..=((vars.res.0 * (1.0 / delta_x) as i32)/ vars.res.2) {
        let x = -i as f32 * delta_x;
        let x_pixel = x * vars.res.2 as f32 + xb;
        let y_pixel = -math_equation(x) * vars.res.2 as f32 + yb;

        let xn_pixel = x * vars.res.2 as f32 + delta_x + xb;
        let yn_pixel = -math_equation(x + delta_x) * vars.res.2 as f32 + yb;
        vars.cvs.draw_line((x_pixel as i32, y_pixel as i32), (xn_pixel as i32, yn_pixel as i32)).unwrap();
    }
    // positive x
    for i in 0..=((vars.res.0 * (1.0 / delta_x) as i32) / vars.res.2) {
        let x = i as f32 * delta_x;
        //let y = calc::equation(x, xb, yb, delta_x);
        let x_pixel = x * vars.res.2 as f32 + xb;
        let y_pixel = -math_equation(x) * vars.res.2 as f32 + yb;

        let xn_pixel = x * vars.res.2 as f32 + delta_x + xb;
        let yn_pixel = -math_equation(x + delta_x) * vars.res.2 as f32 + yb;
        vars.cvs.draw_line((x_pixel as i32, y_pixel as i32), (xn_pixel as i32, yn_pixel as i32)).unwrap();
    }
    vars.cvs.present();
}

fn math_equation(x: f32) -> f32 {
    //x.powi(2)
    2.71_f32.powf(x)
}
