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

fn path_format(top: &str, dut: &str, path: &str) -> String {
    format!("{}.{}.{}", top, dut, path)
}

#[derive(Clone, Debug)]
pub struct Resource {
    pub id: u32,
    pub width: u32,
    pub path: String,
}

impl Resource {
    pub fn new(id: u32, width: u32, path: &str) -> Resource {
        Resource {
            id,
            width,
            path: path.to_string(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub name: String,
    pub registers: Vec<Resource>,
    pub memories: Vec<Resource>,
}

impl Default for Interface {
    fn default() -> Interface {
        Interface {
            name: String::new(),
            registers: Vec::new(),
            memories: Vec::new(),
        }
    }
}

impl Interface {
    pub fn new(name: &str) -> Interface {
        Interface {
            name: name.to_string(),
            registers: Vec::new(),
            memories: Vec::new(),
        }
    }

    pub fn add_register(&mut self, id: u32, width: u32, path: &str) {
        self.registers.push(Resource::new(id, width, path));
    }

    pub fn add_memory(&mut self, id: u32, width: u32, path: &str) {
        self.memories.push(Resource::new(id, width, path));
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn registers(&self) -> &Vec<Resource> {
        &self.registers
    }

    pub fn memories(&self) -> &Vec<Resource> {
        &self.memories
    }

    pub fn emit_module(&self) -> Module {
        let mut module = Module::new_with_name(&self.name());
        module.add_input("clock", 1);
        module.add_input("reset", 1);
        module.add_input("opcode", 32);
        module.add_input("id", 32);
        module.add_input("mask", 32);
        module.add_input("in", 32);
        module.add_input("addr", 32);
        module.add_output("out", 32);
        for reg in self.registers().iter() {
            module.add_function(func_write_reg(
                reg.id(),
                reg.width(),
                &path_format(&self.name, "dut", &reg.path()),
            ));
            module.add_function(func_read_reg(
                reg.id(),
                reg.width(),
                &path_format(&self.name, "dut", &reg.path()),
            ));
        }
        for mem in self.memories().iter() {
            module.add_function(func_write_mem(
                mem.id(),
                mem.width(),
                &path_format(&self.name, "dut", &mem.path()),
            ));
            module.add_function(func_read_mem(
                mem.id(),
                mem.width(),
                &path_format(&self.name, "dut", &mem.path()),
            ));
        }
        module
    }
}

fn main() {
    let mut interface = Interface::new("testbench");
    interface.add_register(
        0,
        1,
        "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start",
    );
    interface.add_register(
        1,
        1,
        "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done",
    );
    interface.add_register(
        2,
        64,
        "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a",
    );
    interface.add_register(
        3,
        64,
        "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b",
    );
    interface.add_register(
        4,
        64,
        "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c",
    );
    interface.add_register(
        5,
        32,
        "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r",
    );
    interface.add_memory(0, 32, "ram.mem");
    println!("{}", interface.emit_module());
}
