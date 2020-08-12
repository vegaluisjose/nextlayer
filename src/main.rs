use vast::v17::ast::*;

fn round_to_chunk(width: u32) -> u32 {
    if width % 32 == 0 {
        width
    } else {
        ((width / 32) + 1) * 32
    }
}

fn max_mask_val(width: u32) -> u32 {
    if width % 32 == 0 {
        width / 32
    } else {
        (width / 32) + 1
    }
}

fn slice(var: &str, hi: u32, lo: u32) -> Expr {
    Expr::new_slice(var, Expr::new_int(hi as i32), Expr::new_int(lo as i32))
}

fn bit(var: &str, bit: u32) -> Expr {
    Expr::new_bit(var, bit as i32)
}

fn mask_slice(var: &str, mask: &str) -> Expr {
    let width = 32;
    let lo = Expr::new_mul(Expr::new_int(width), Expr::new_ref(mask));
    Expr::new_index_slice(var, lo, width as u32)
}

fn mask_check(mask: &str, width: u32) -> Sequential {
    let cond = Expr::new_lt(
        Expr::new_ref(mask),
        Expr::new_int(max_mask_val(width) as i32),
    );
    let err = Sequential::new_error("mask out of bounds");
    Sequential::new_assert_with_else(cond, err)
}

fn write_mask(var: &str, mask: &str, val: &str) -> Sequential {
    Sequential::new_blk_assign(mask_slice(var, mask), Expr::new_ref(val))
}

fn clear_var(var: &str, width: u32) -> Sequential {
    Sequential::new_blk_assign(Expr::new_ref(var), Expr::new_ulit_dec(width, "0"))
}

fn read_path(var: &str, width: u32, path: Expr) -> Sequential {
    assert!(width > 0, "Error: width must be greater than zero");
    if width > 1 {
        Sequential::new_blk_assign(slice(var, width - 1, 0), path)
    } else {
        Sequential::new_blk_assign(bit(var, width - 1), path)
    }
}

fn write_path(var: &str, width: u32, path: Expr) -> Sequential {
    assert!(width > 0, "Error: width must be greater than zero");
    if width > 1 {
        Sequential::new_blk_assign(path, slice(var, width - 1, 0))
    } else {
        Sequential::new_blk_assign(path, bit(var, width - 1))
    }
}

fn return_mask(var: &str, mask: &str) -> Sequential {
    Sequential::new_return(mask_slice(var, mask))
}

fn func_write_reg(id: u32, width: u32, path: &str) -> Function {
    let name = format!("write_reg_{}", id);
    let path = Expr::new_ipath(path);
    let mut func = Function::new(&name, Ty::Void);
    func.add_input("value", 32);
    func.add_input("mask", 32);
    func.add_logic("tmp", round_to_chunk(width) as u64);
    func.add_stmt(mask_check("mask", width));
    func.add_stmt(clear_var("tmp", width));
    func.add_stmt(read_path("tmp", width, path.clone()));
    func.add_stmt(write_mask("tmp", "mask", "value"));
    func.add_stmt(write_path("tmp", width, path));
    func
}

fn func_read_reg(id: u32, width: u32, path: &str) -> Function {
    let name = format!("read_reg_{}", id);
    let path = Expr::new_ipath(path);
    let mut func = Function::new(&name, Ty::Int);
    func.add_input("mask", 32);
    func.add_logic("tmp", 32);
    func.add_stmt(mask_check("mask", width));
    func.add_stmt(clear_var("tmp", width));
    func.add_stmt(read_path("tmp", width, path));
    func.add_stmt(return_mask("tmp", "mask"));
    func
}

// function void write_mem_0;
// input int value;
// input int addr;
// input int mask;
// logic [32-1:0] tmp;
// begin
//     assert (mask < 1) else $error("mask out of bounds");
//     tmp[0+:32] = 0;
//     tmp[0+:32] = testbench.dut.ram.mem[addr];
//     tmp[mask*32+:32] = value;
//     testbench.dut.ram.mem[addr] = tmp[0+:32];
// end
// endfunction

fn func_write_mem(id: u32, width: u32, path: &str) -> Function {
    let name = format!("write_mem_{}", id);
    let path = Expr::new_ipath_with_index(path, "addr");
    let mut func = Function::new(&name, Ty::Void);
    func.add_input("value", 32);
    func.add_input("addr", 32);
    func.add_input("mask", 32);
    func.add_logic("tmp", round_to_chunk(width) as u64);
    func.add_stmt(mask_check("mask", width));
    func.add_stmt(clear_var("tmp", width));
    func.add_stmt(read_path("tmp", width, path.clone()));
    func.add_stmt(write_mask("tmp", "mask", "value"));
    func.add_stmt(write_path("tmp", width, path));
    func
}

fn func_read_mem(id: u32, width: u32, path: &str) -> Function {
    let name = format!("read_mem_{}", id);
    let path = Expr::new_ipath_with_index(path, "addr");
    let mut func = Function::new(&name, Ty::Int);
    func.add_input("addr", 32);
    func.add_input("mask", 32);
    func.add_logic("tmp", 32);
    func.add_stmt(mask_check("mask", width));
    func.add_stmt(clear_var("tmp", width));
    func.add_stmt(read_path("tmp", width, path));
    func.add_stmt(return_mask("tmp", "mask"));
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
    module.add_function(func_write_reg(
        0,
        1,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start",
    ));
    module.add_function(func_read_reg(
        0,
        1,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start",
    ));
    module.add_function(func_write_reg(
        1,
        1,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done",
    ));
    module.add_function(func_read_reg(
        1,
        1,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done",
    ));
    module.add_function(func_write_reg(
        2,
        64,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a",
    ));
    module.add_function(func_read_reg(
        2,
        64,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a",
    ));
    module.add_function(func_write_reg(
        3,
        64,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b",
    ));
    module.add_function(func_read_reg(
        3,
        64,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b",
    ));
    module.add_function(func_write_reg(
        4,
        64,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c",
    ));
    module.add_function(func_read_reg(
        4,
        64,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c",
    ));
    module.add_function(func_write_reg(
        5,
        32,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r",
    ));
    module.add_function(func_read_reg(
        5,
        32,
        "testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r",
    ));
    module.add_function(func_write_mem(0, 32, "testbench.dut.ram.mem"));
    module.add_function(func_read_mem(0, 32, "testbench.dut.ram.mem"));
    module
}

fn main() {
    println!("{}", module());
}
