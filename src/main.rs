use vast::v17::ast::*;

fn round_width(width: u64) -> u64 {
    if width % 32 == 0 {
        width
    } else {
        ((width / 32) + 1) * 32
    }
}

// fn max_mask_value(width: u64) -> u64 {
//     if width % 32 == 0 {
//         width / 32
//     } else {
//         (width / 32) + 1
//     }
// }

// function void write_reg_0;
// input logic [31:0] value;
// input logic [31:0] mask;
// logic [32-1:0] tmp;
// begin
//     assert (mask < 1) else $error("mask out of bounds");
//     tmp[0+:32] = 0;
//     tmp[0+:1] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start;
//     tmp[mask*32+:32] = value;
//     testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start = tmp[0+:1];
// end
fn func_write_reg(name: &str, width: u64) -> Function {
    let mut func = Function::new(name, Ty::Void);
    func.add_input("value", 32);
    func.add_input("mask", 32);
    func.add_logic("tmp", round_width(width));
    func
}

fn module() -> Module {
    let mut module = Module::new_with_name("nextlayer");
    module.add_input("clock", 1);
    module.add_input("reset", 1);
    module.add_input("opcode", 32);
    module.add_input("id", 32);
    module.add_input("mask", 32);
    module.add_input("in", 32);
    module.add_input("addr", 32);
    module.add_output("out", 32);
    module.add_function(func_write_reg("write_reg_0", 34));
    module
}

fn main() {
    println!("{}", module());
}