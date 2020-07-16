pub mod backend;

use crate::backend::xsim::Xsim;
use std::path::Path;

#[no_mangle]
pub extern "C" fn run_add() {
    println!("Running add...");
    let design_lib_path = Path::new("designs/add/xsim.dir/work.testbench/xsimk.so");
    let mut sim = Xsim::new(&design_lib_path);
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
    sim.eval();
    // write mem
    sim.poke("opcode", 3);
    sim.poke("id", 0);
    sim.poke("in", 9);
    sim.poke("addr", 4);
    sim.eval();
    // read mem
    sim.poke("opcode", 4);
    sim.poke("id", 0);
    sim.poke("addr", 4);
    sim.eval();
    println!("read-back from mem:{}", sim.peek("out"));
    // run for 1 cycle
    for _ in 0..1 {
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    // read y
    sim.poke("opcode", 2);
    sim.poke("id", 1);
    sim.eval();
    println!("adder result:{}", sim.peek("out"));
    println!("Finishing add...");
}
