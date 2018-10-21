#![allow(dead_code)]

mod program;

use program::Program;

fn main() {
    let p = Program::new();
    p.do_nothing();
}
