pub mod backend;

use crate::backend::xsim::Xsim;

#[no_mangle]
pub extern fn run_xsim() {
    println!("Running...");
    let mut sim = Xsim::new();
    // reset for 10 cycles
    sim.poke("mask", 0);
    for _ in 0..10 {
        sim.poke("reset", 1);
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    sim.poke("reset", 0);
    // write a
    sim.poke("opcode", 1);
    sim.poke("id", 0);
    sim.poke("in", 3);
    sim.eval();
    // write b
    sim.poke("opcode", 1);
    sim.poke("id", 1);
    sim.poke("in", 4);
    sim.eval();
    // run for 1 cycle
    for _ in 0..1 {
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    // read y
    sim.poke("opcode", 2);
    sim.poke("id", 2);
    sim.eval();
    println!("adder result:{}", sim.peek("out"));
    println!("Finishing...");
}
