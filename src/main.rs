use vast::v17::ast::*;

fn path_format(top: &str, dut: &str, path: &str) -> String {
    format!("{}.{}.{}", top, dut, path)
}

fn write_reg_format(id: u32) -> String {
    format!("write_reg_{}", id)
}

fn read_reg_format(id: u32) -> String {
    format!("read_reg_{}", id)
}

fn write_mem_format(id: u32) -> String {
    format!("write_mem_{}", id)
}

fn read_mem_format(id: u32) -> String {
    format!("read_mem_{}", id)
}

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
    let path = Expr::new_ipath(path);
    let mut func = Function::new(&write_reg_format(id), Ty::Void);
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
    let path = Expr::new_ipath(path);
    let mut func = Function::new(&read_reg_format(id), Ty::Int);
    func.add_input("mask", 32);
    func.add_logic("tmp", 32);
    func.add_stmt(mask_check("mask", width));
    func.add_stmt(clear_var("tmp", width));
    func.add_stmt(read_path("tmp", width, path));
    func.add_stmt(return_mask("tmp", "mask"));
    func
}

fn func_write_mem(id: u32, width: u32, path: &str) -> Function {
    let path = Expr::new_ipath_with_index(path, "addr");
    let mut func = Function::new(&write_mem_format(id), Ty::Void);
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
    let path = Expr::new_ipath_with_index(path, "addr");
    let mut func = Function::new(&read_mem_format(id), Ty::Int);
    func.add_input("addr", 32);
    func.add_input("mask", 32);
    func.add_logic("tmp", 32);
    func.add_stmt(mask_check("mask", width));
    func.add_stmt(clear_var("tmp", width));
    func.add_stmt(read_path("tmp", width, path));
    func.add_stmt(return_mask("tmp", "mask"));
    func
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

    pub fn emit_func_write_reg(&self) -> Vec<Function> {
        let mut func: Vec<Function> = Vec::new();
        for reg in self.registers().iter() {
            func.push(func_write_reg(
                reg.id(),
                reg.width(),
                &path_format(&self.name, "dut", &reg.path()),
            ));
        }
        func
    }

    pub fn emit_func_read_reg(&self) -> Vec<Function> {
        let mut func: Vec<Function> = Vec::new();
        for reg in self.registers().iter() {
            func.push(func_read_reg(
                reg.id(),
                reg.width(),
                &path_format(&self.name, "dut", &reg.path()),
            ));
        }
        func
    }

    pub fn emit_func_write_mem(&self) -> Vec<Function> {
        let mut func: Vec<Function> = Vec::new();
        for reg in self.memories().iter() {
            func.push(func_write_mem(
                reg.id(),
                reg.width(),
                &path_format(&self.name, "dut", &reg.path()),
            ));
        }
        func
    }

    pub fn emit_func_read_mem(&self) -> Vec<Function> {
        let mut func: Vec<Function> = Vec::new();
        for reg in self.memories().iter() {
            func.push(func_read_mem(
                reg.id(),
                reg.width(),
                &path_format(&self.name, "dut", &reg.path()),
            ));
        }
        func
    }

    pub fn emit_case_write_reg(&self) -> Case {
        let mut case = Case::new(Expr::new_ref("id"));
        for reg in self.registers().iter() {
            let mut br = CaseBranch::new(Expr::new_ulit_dec(32, &reg.id().to_string()));
            br.add_stmt(Sequential::new_display(&write_reg_format(reg.id())));
            case.add_branch(br);
        }
        let mut default = CaseDefault::default();
        default.add_stmt(Sequential::new_error("invalid id"));
        case.set_default(default);
        case
    }

    pub fn emit_case_read_reg(&self) -> Case {
        let mut case = Case::new(Expr::new_ref("id"));
        for reg in self.registers().iter() {
            let mut br = CaseBranch::new(Expr::new_ulit_dec(32, &reg.id().to_string()));
            br.add_stmt(Sequential::new_display(&read_reg_format(reg.id())));
            case.add_branch(br);
        }
        let mut default = CaseDefault::default();
        default.add_stmt(Sequential::new_error("invalid id"));
        case.set_default(default);
        case
    }

    pub fn emit_case_write_mem(&self) -> Case {
        let mut case = Case::new(Expr::new_ref("id"));
        for reg in self.memories().iter() {
            let mut br = CaseBranch::new(Expr::new_ulit_dec(32, &reg.id().to_string()));
            br.add_stmt(Sequential::new_display(&write_mem_format(reg.id())));
            case.add_branch(br);
        }
        let mut default = CaseDefault::default();
        default.add_stmt(Sequential::new_error("invalid id"));
        case.set_default(default);
        case
    }

    pub fn emit_case_read_mem(&self) -> Case {
        let mut case = Case::new(Expr::new_ref("id"));
        for reg in self.memories().iter() {
            let mut br = CaseBranch::new(Expr::new_ulit_dec(32, &reg.id().to_string()));
            br.add_stmt(Sequential::new_display(&read_mem_format(reg.id())));
            case.add_branch(br);
        }
        let mut default = CaseDefault::default();
        default.add_stmt(Sequential::new_error("invalid id"));
        case.set_default(default);
        case
    }

    pub fn emit_case(&self) -> Case {
        let mut case = Case::new(Expr::new_ref("opcode"));
        let mut nop = CaseBranch::new(Expr::new_ulit_dec(32, "0"));
        let mut write_reg = CaseBranch::new(Expr::new_ulit_dec(32, "1"));
        let mut read_reg = CaseBranch::new(Expr::new_ulit_dec(32, "2"));
        let mut write_mem = CaseBranch::new(Expr::new_ulit_dec(32, "3"));
        let mut read_mem = CaseBranch::new(Expr::new_ulit_dec(32, "4"));
        let mut default = CaseDefault::default();
        nop.add_stmt(Sequential::new_blk_assign(
            Expr::new_ref("out"),
            Expr::new_ulit_hex(32, "deadbeef"),
        ));
        write_reg.add_stmt(Sequential::new_case(self.emit_case_write_reg()));
        read_reg.add_stmt(Sequential::new_case(self.emit_case_read_reg()));
        write_mem.add_stmt(Sequential::new_case(self.emit_case_write_mem()));
        read_mem.add_stmt(Sequential::new_case(self.emit_case_read_mem()));
        default.add_stmt(Sequential::new_error("invalid opcode"));
        case.add_branch(nop);
        case.add_branch(write_reg);
        case.add_branch(read_reg);
        case.add_branch(write_mem);
        case.add_branch(read_mem);
        case.set_default(default);
        case
    }

    pub fn emit_always(&self) -> AlwaysComb {
        let mut always = AlwaysComb::default();
        always.add_case(self.emit_case());
        always
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
        for func in self.emit_func_write_reg().iter() {
            module.add_function(func.clone());
        }
        for func in self.emit_func_read_reg().iter() {
            module.add_function(func.clone());
        }
        for func in self.emit_func_write_mem().iter() {
            module.add_function(func.clone());
        }
        for func in self.emit_func_read_mem().iter() {
            module.add_function(func.clone());
        }
        module.add_always_comb(self.emit_always());
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
