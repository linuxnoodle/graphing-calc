pub mod engine;

use engine::*;
use std::time::Instant;

fn main() {
    let mut vars = init((800, 600, 10));
    loop {
        if !vars.running_state { break; }
        update(&mut vars);
        render(&mut vars);
    }
}
