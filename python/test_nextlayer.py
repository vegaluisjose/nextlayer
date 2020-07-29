import sys
from os import environ, getenv, path
from nextlayer import NextLayer

def add(nextlayer_lib):
    design_lib = "designs/add/xsim.dir/work.testbench/xsimk.so"
    nextlayer = NextLayer(nextlayer_lib, design_lib)

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    sys.path.append(cur_dir)
    add(nextlayer_lib)
