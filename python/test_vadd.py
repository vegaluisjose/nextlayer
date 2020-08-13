import sys
from os import environ, getenv, path
from driver import VaddDriver

def test_vadd(nextlayer_lib, design_lib):
    # n is vector length
    n = 16
    # vector size in bytes, word type is 32-bit
    size = n*4
    # create a driver
    driver = VaddDriver(nextlayer_lib, design_lib)
    # reset accel for 10 cycles
    driver.reset(10)
    # write vector a
    for x in range(n):
        driver.write_mem(x, x)
    # write vector b
    for x in range(n):
        driver.write_mem(x+3, x+n)
    # write pointer a
    driver.write_reg_a(0)
    # write pointer b
    driver.write_reg_b(size)
    # write pointer c
    driver.write_reg_c(size*2)
    # write length
    driver.write_reg_length(n)
    # launch
    driver.launch()
    # cycle counter
    cycles = 0
    # run accel, timeout set to 100000 cycles
    for i in range(100000):
        cycles += 1
        driver.run(1)
        if driver.is_finished():
            break
    # check results
    for x in range(n):
        a = driver.read_mem(x)
        b = driver.read_mem(x+n)
        c = driver.read_mem(x+n*2)
        assert (a + b) == c
    # print cycles
    print("total cycles:{}".format(cycles))

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    design_lib = path.join(cur_dir, "../hw/vadd/xsim.dir/work.testbench/xsimk.so")
    sys.path.append(cur_dir)
    test_vadd(nextlayer_lib, design_lib)
