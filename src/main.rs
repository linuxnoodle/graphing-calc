pub mod engine;

use engine::*;

fn main() {
    let mut vars = init((1280, 720, 20));
    loop {
        if !vars.running_state { break; }
        update(&mut vars);
        render(&mut vars);
    }
}
