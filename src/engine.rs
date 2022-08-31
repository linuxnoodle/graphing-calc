extern crate sdl2;
extern crate egui;

pub mod calc;
pub mod converter;

//use std::time::Instant;
//use egui_sdl2_gl as egui_backend;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use calc::*;
//use converter::*;

pub struct EngineVars {
    ctx: sdl2::Sdl,
    cvs: sdl2::render::Canvas<sdl2::video::Window>,
    //winr: Box<sdl2::video::Window>,
    //ectx: egui::CtxRef,
    //estate: egui_backend::EguiStateHandler,
    res: (i32, i32, i32), // x, y, unit size
    grid_offset: (i32, i32),
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
    
    // egui init
    /*let shader_ver = egui_backend::ShaderVersion::Adaptive;
    let (_painter, egui_state) = 
        egui_backend::with_sdl2(&sdl_window, shader_ver, egui_backend::DpiScaling::Custom(2.0));
    let egui_ctx = egui::CtxRef::default();*/

    let sdl_canvas = sdl_window.into_canvas().build().unwrap();
    Box::new(EngineVars {
        ctx: sdl_context,
        cvs: sdl_canvas,
        //winr: sdl_window,
        //ectx: egui_ctx,
        //estate: egui_state,
        res: size,
        grid_offset: (0, 0),
        running_state: true
    })
}

pub fn update(vars: &mut Box<EngineVars>) {
    let mut event_pump = vars.ctx.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                vars.running_state = false;
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                vars.grid_offset.1 += 5;
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                vars.grid_offset.1 -= 5;
            }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                vars.grid_offset.0 += 5;
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                vars.grid_offset.0 -= 5;
            }
            _ => {}
        }
    }

}
    
pub fn render(vars: &mut Box<EngineVars>) {
    // egui rendering
    /*let mut _egui_output = vars.ectx.begin_frame(vars.estate.input.take());
    egui::CentralPanel::default().show(&vars.ectx, |ui| {
        ui.heading("Hello World!");
    });
    let (_egui_output, _paint_cmds) = vars.ectx.end_frame();*/
    vars.cvs.set_draw_color(Color::RGB(0, 0, 0));
    vars.cvs.clear();

    vars.cvs.set_draw_color(Color::RGB(20, 120, 150));
    let (grid_off_xn, grid_off_yn) = (vars.grid_offset.0 % vars.res.2, vars.grid_offset.1 % vars.res.2);
    // vertical gridlines
    for i in 0..=(vars.res.0 / vars.res.2) {
        vars.cvs.draw_line((i * vars.res.2 + grid_off_xn, 0),
                           (i * vars.res.2 + grid_off_xn, vars.res.1)).unwrap();
    }
    // horizontal gridlines
    for i in 0..=(vars.res.1 / vars.res.2) {
        vars.cvs.draw_line((0, i * vars.res.2 + grid_off_yn),
                           (vars.res.0, i * vars.res.2 + grid_off_yn)).unwrap();
    }

    // overdraw axes because it's faster than an if statement
    vars.cvs.set_draw_color(Color::RGB(100, 200, 230));
    vars.cvs.draw_line((vars.res.0 / 2 + vars.grid_offset.0, 0),
                       (vars.res.0 / 2 + vars.grid_offset.0, vars.res.1)).unwrap();
    vars.cvs.draw_line((0, vars.res.1 / 2 + vars.grid_offset.1),
                       (vars.res.0, vars.res.1 / 2 + vars.grid_offset.1)).unwrap();

    // draw equation listed 
    // TODO: tie in with equation parser to run any equation
    vars.cvs.set_draw_color(Color::RGB(255, 255, 255));
    let delta_x = 0.01;
    let xb = vars.res.0 as f32 / 2.0;
    let yb = vars.res.1 as f32 / 2.0;

    let iterations = (vars.res.0 * (1.0 / delta_x) as i32)/ vars.res.2;
    for i in -iterations/2..iterations/2 {
        let x = i as f32 * delta_x - vars.grid_offset.0 as f32 / vars.res.2 as f32;
        let x_pixel = x * vars.res.2 as f32 + xb as f32 + vars.grid_offset.0 as f32;
        let y_pixel = -math_equation(x) * vars.res.2 as f32 + yb as f32 + vars.grid_offset.1 as f32;

        let xn_pixel = x * vars.res.2 as f32 + delta_x + xb as f32 + vars.grid_offset.0 as f32;
        let yn_pixel = -math_equation(x + delta_x) * vars.res.2 as f32 + yb as f32 + vars.grid_offset.1 as f32;
        vars.cvs.draw_line((x_pixel as i32, y_pixel as i32),
                           (xn_pixel as i32, yn_pixel as i32)).unwrap();
    }
    vars.cvs.present();
}

fn math_equation(x: f32) -> f32 {
    0.5 * x * x.sin()
}
