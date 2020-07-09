pub mod backend;

use crate::backend::xsim::Xsim;

#[no_mangle]
pub extern fn run_xsim() {
    println!("Running...");
    let mut sim = Xsim::new();
    for _ in 0..10 {
        sim.poke("reset", 1);
        sim.poke("clock", 0);
        sim.eval();
        sim.poke("clock", 1);
        sim.eval();
    }
    sim.poke("reset", 0);
    for _ in 0..10 {
        sim.poke("clock", 0);
        sim.eval();
        sim.poke("clock", 1);
        sim.eval();
        println!("count:{}", sim.peek("count"));
    }
    println!("Finishing...");
}
