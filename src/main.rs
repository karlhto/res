use std::io;
mod cpu;

/*
 * How to structure:
 *  1.
 *
 */
fn main() {
    // TODO Implement command line arguments
    // TODO Initialise CPU state
    let mut memory: [i8; 65536] = [0; 65536]; // probably going to be replaced
    let mut state = cpu::cpu_state::cpu_init(&memory);
}

