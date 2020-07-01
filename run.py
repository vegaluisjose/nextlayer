from os import environ, getenv, path
from ctypes import CDLL, c_int

cur_dir = path.dirname(path.realpath(__file__))
sim_lib = path.join(cur_dir, "target/release/liblastlayer.so")

lib = CDLL(sim_lib)

lib.wrapper_add.restype = c_int
lib.wrapper_add.argtypes = [c_int, c_int]

print("wrapper:", lib.wrapper_add(3, 4))
