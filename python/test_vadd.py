import sys
from os import environ, getenv, path
from driver import VaddDriver

def test_vadd(nextlayer_lib, design_lib):
    # create a driver
    driver = VaddDriver(nextlayer_lib, design_lib)
    # reset accel for 10 cycles
    driver.reset(10)
    # write a and b vector to mem
    for x in range(8):
        driver.write_mem(x, x)
    # write pointer a addr
    driver.write_reg_a(0)
    # write pointer b addr
    driver.write_reg_b(16)
    # write pointer c addr
    driver.write_reg_c(32)
    # write length
    driver.write_reg_length(4)
    # launch
    driver.launch()
    # cycle counter
    cycles = 0
    # run accel, timeout set to 1000 cycles
    for i in range(1000):
        cycles += 1
        driver.run(1)
        if driver.is_finished():
            break
    # read a, b, and c vector from mem
    for x in range(12):
        print("mem[{}]:{}".format(x, driver.read_mem(x)))
    # print cycles
    print("total cycles:{}".format(cycles))

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    design_lib = path.join(cur_dir, "../designs/vadd/xsim.dir/work.testbench/xsimk.so")
    sys.path.append(cur_dir)
    test_vadd(nextlayer_lib, design_lib)
