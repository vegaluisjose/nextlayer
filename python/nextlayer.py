from ctypes import CDLL, c_int, c_void_p, c_char_p

def to_bytes(path_str):
    return bytes(path_str, encoding="ascii")

class NextLayer:
    def __init__(self, nextlayer_lib, design_lib):
        self.lib = CDLL(nextlayer_lib)
        self.lib.alloc.argtypes = [c_char_p]
        self.lib.alloc.restype = c_void_p
        self.lib.write_reg.argtypes = [c_void_p, c_int, c_int, c_int]
        self.lib.read_reg.argtypes = [c_void_p, c_int, c_int]
        self.lib.read_reg.restype = c_int
        self.lib.dealloc.argtypes = [c_void_p]
        self.handle = self.lib.alloc(to_bytes(design_lib))

    def __del__(self):
        self.lib.dealloc(self.handle)

    def write_reg(self, value):
        self.lib.write_reg(self.handle, value, 0, 0)

    def read_reg(self):
        return self.lib.read_reg(self.handle, 0, 0)
