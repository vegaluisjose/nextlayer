import sys
from os import environ, getenv, path
from driver import VaddDriver

def test_vadd(nextlayer_lib, design_lib):
    driver = VaddDriver(nextlayer_lib, design_lib)
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
    # start
    driver.write_reg_start(1)  # start
    # run for 1000 cycles
    driver.run(1000)
    # check if done
    print("done:{}".format(driver.read_reg_done()))
    # read a, b, and c vector from mem
    for x in range(12):
        print("mem[{}]:{}".format(x, driver.read_mem(x)))

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    design_lib = path.join(cur_dir, "../designs/vadd/xsim.dir/work.testbench/xsimk.so")
    sys.path.append(cur_dir)
    test_vadd(nextlayer_lib, design_lib)
