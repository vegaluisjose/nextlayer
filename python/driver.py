from ctypes import CDLL, c_int, c_void_p, c_char_p

def to_bytes(path_str):
    return bytes(path_str, encoding="ascii")

class AddDriver:
    def __init__(self, nextlayer_lib, design_lib):
        self.lib = CDLL(nextlayer_lib)
        self.lib.alloc.argtypes = [c_char_p]
        self.lib.alloc.restype = c_void_p
        self.lib.run.argtypes = [c_int]
        self.lib.reset.argtypes = [c_int]
        self.lib.write_reg.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_reg.argtypes = [c_void_p, c_int, c_int]
        self.lib.read_reg.restype = c_int
        self.lib.write_mem.argtypes = [c_void_p, c_int, c_int, c_int, c_int]
        self.lib.read_mem.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_mem.restype = c_int
        self.lib.dealloc.argtypes = [c_void_p]
        self.handle = self.lib.alloc(to_bytes(design_lib))

    def __del__(self):
        self.lib.dealloc(self.handle)

    def reset(self, cycles):
        self.lib.reset(self.handle, cycles)

    def run(self, cycles):
        self.lib.run(self.handle, cycles)

    def write_reg_a(self, value):
        self.lib.write_reg(self.handle, value, 0, 0)

    def read_reg_a(self):
        return self.lib.read_reg(self.handle, 0, 0)

    def write_reg_y(self, value):
        self.lib.write_reg(self.handle, value, 1, 0)

    def read_reg_y(self):
        return self.lib.read_reg(self.handle, 1, 0)

    def write_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 0, 0)

    def read_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 0, 0)

class VaddDriver:
    def __init__(self, nextlayer_lib, design_lib):
        self.lib = CDLL(nextlayer_lib)
        self.lib.alloc.argtypes = [c_char_p]
        self.lib.alloc.restype = c_void_p
        self.lib.run.argtypes = [c_int]
        self.lib.reset.argtypes = [c_int]
        self.lib.write_reg.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_reg.argtypes = [c_void_p, c_int, c_int]
        self.lib.read_reg.restype = c_int
        self.lib.write_mem.argtypes = [c_void_p, c_int, c_int, c_int, c_int]
        self.lib.read_mem.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_mem.restype = c_int
        self.lib.dealloc.argtypes = [c_void_p]
        self.handle = self.lib.alloc(to_bytes(design_lib))

    def __del__(self):
        self.lib.dealloc(self.handle)

    def reset(self, cycles):
        self.lib.reset(self.handle, cycles)

    def run(self, cycles):
        self.lib.run(self.handle, cycles)

    def launch(self):
        self.lib.write_reg(self.handle, 1, 0, 0)

    def is_finished(self):
        return self.lib.read_reg(self.handle, 1, 0)

    def write_reg_a(self, value):
        self.lib.write_reg(self.handle, value, 2, 0)

    def read_reg_a(self):
        return self.lib.read_reg(self.handle, 2, 0)

    def write_reg_b(self, value):
        self.lib.write_reg(self.handle, value, 3, 0)

    def read_reg_b(self):
        return self.lib.read_reg(self.handle, 3, 0)

    def write_reg_c(self, value):
        self.lib.write_reg(self.handle, value, 4, 0)

    def read_reg_c(self):
        return self.lib.read_reg(self.handle, 4, 0)

    def write_reg_length(self, value):
        self.lib.write_reg(self.handle, value, 5, 0)

    def read_reg_length(self):
        return self.lib.read_reg(self.handle, 5, 0)

    def write_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 0, 0)

    def read_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 0, 0)

class FifoDriver:
    def __init__(self, nextlayer_lib, design_lib):
        self.lib = CDLL(nextlayer_lib)
        self.lib.alloc.argtypes = [c_char_p]
        self.lib.alloc.restype = c_void_p
        self.lib.run.argtypes = [c_int]
        self.lib.reset.argtypes = [c_int]
        self.lib.write_mem.argtypes = [c_void_p, c_int, c_int, c_int, c_int]
        self.lib.read_mem.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_mem.restype = c_int
        self.lib.dealloc.argtypes = [c_void_p]
        self.handle = self.lib.alloc(to_bytes(design_lib))

    def __del__(self):
        self.lib.dealloc(self.handle)

    def reset(self, cycles):
        self.lib.reset(self.handle, cycles)

    def run(self, cycles):
        self.lib.run(self.handle, cycles)

    def write_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 0, 0)

    def read_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 0, 0)

class FetchDriver:
    def __init__(self, nextlayer_lib, design_lib):
        self.lib = CDLL(nextlayer_lib)
        self.lib.alloc.argtypes = [c_char_p]
        self.lib.alloc.restype = c_void_p
        self.lib.run.argtypes = [c_int]
        self.lib.reset.argtypes = [c_int]
        self.lib.write_reg.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_reg.argtypes = [c_void_p, c_int, c_int]
        self.lib.read_reg.restype = c_int
        self.lib.write_mem.argtypes = [c_void_p, c_int, c_int, c_int, c_int]
        self.lib.read_mem.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_mem.restype = c_int
        self.lib.dealloc.argtypes = [c_void_p]
        self.handle = self.lib.alloc(to_bytes(design_lib))

    def __del__(self):
        self.lib.dealloc(self.handle)

    # reset for 10 cycles
    def reset(self):
        self.lib.reset(self.handle, 10)

    # run for one cycle
    def run(self):
        self.lib.run(self.handle, 1)

    def launch(self):
        self.lib.write_reg(self.handle, 1, 0, 0)

    def is_finished(self):
        return self.lib.read_reg(self.handle, 1, 0)

    def write_instr_addr(self, value):
        self.lib.write_reg(self.handle, value, 2, 0)

    def read_instr_addr(self):
        return self.lib.read_reg(self.handle, 2, 0)

    def write_instr_count(self, value):
        self.lib.write_reg(self.handle, value, 3, 0)

    def read_instr_count(self):
        return self.lib.read_reg(self.handle, 3, 0)

    def write_instr_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 0, 0)

    def read_instr_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 0, 0)

    def write_load_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 1, 0)

    def read_load_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 1, 0)

    def write_gemm_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 2, 0)

    def read_gemm_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 2, 0)

    def write_store_mem(self, value, addr):
        self.lib.write_mem(self.handle, value, addr, 3, 0)

    def read_store_mem(self, addr):
        return self.lib.read_mem(self.handle, addr, 3, 0)