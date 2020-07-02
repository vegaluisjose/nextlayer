from os import environ, getenv, path
from ctypes import CDLL, c_int

cur_dir = path.dirname(path.realpath(__file__))
sim_lib = path.join(cur_dir, "target/release/liblastlayer.so")

lib = CDLL(sim_lib)

lib.run_xsim()
