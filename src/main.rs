pub mod engine;

use engine::*;

fn main() {
    let mut vars = init((1920, 1080, 10));
    loop {
        if !vars.running_state { break; }
        update(&mut vars);
        render(&mut vars);
    }
}
