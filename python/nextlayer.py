from ctypes import CDLL, c_int, c_void_p, c_char_p

def to_bytes(path_str):
    return bytes(path_str, encoding="ascii")

class NextLayer:
    def __init__(self, nextlayer_lib, design_lib):
        self.lib = CDLL(nextlayer_lib)
        #self.alloc = self.lib.alloc
        #self.dealloc = self.lib.dealloc
        self.lib.alloc.argtypes = [c_char_p]
        self.lib.alloc.restype = c_void_p
        #self.dealloc.argtypes = [c_void_p]
        self.handle = self.lib.alloc(to_bytes(design_lib))
    
    #def __del__(self):
    #    self.dealloc(self.handle)
