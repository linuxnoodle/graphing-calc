pub mod engine;

use engine::*;

fn main() {
    let mut vars = init((800, 600, 20));
    loop {
        if !vars.running_state { break; }
        update(&mut vars);
        render(&mut vars);
    }
}
