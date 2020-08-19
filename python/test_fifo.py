import sys
from os import environ, getenv, path
from driver import FifoDriver

def test_fifo(nextlayer_lib, design_lib):
    # create a driver
    driver = FifoDriver(nextlayer_lib, design_lib)
    # reset accel for 10 cycles
    driver.reset(10)
    # run for 10 cycles
    driver.run(10)
    # check fifo memory
    for addr in range(4):
        print("addr:{} value:{}".format(addr, driver.read_mem(addr)))

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    design_lib = path.join(cur_dir, "../hw/fifo/xsim.dir/work.testbench/xsimk.so")
    sys.path.append(cur_dir)
    test_fifo(nextlayer_lib, design_lib)
