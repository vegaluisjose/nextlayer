import sys
from os import environ, getenv, path
from driver import AddDriver

def test_add(nextlayer_lib, design_lib):
    driver = AddDriver(nextlayer_lib, design_lib)
    driver.reset(4)
    driver.write_reg_a(3)
    driver.write_mem(9, 4)
    print("lhs:{}".format(driver.read_reg_a()))
    print("rhs:{}".format(driver.read_mem(4)))
    driver.run(3)
    print("result:{}".format(driver.read_reg_y()))

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    design_lib = path.join(cur_dir, "../designs/add/xsim.dir/work.testbench/xsimk.so")
    sys.path.append(cur_dir)
    test_add(nextlayer_lib, design_lib)
