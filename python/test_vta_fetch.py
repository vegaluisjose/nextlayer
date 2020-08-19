import sys
from os import environ, getenv, path
from driver import FetchDriver

# VTA_OPCODE_LOAD 0
# VTA_OPCODE_STORE 1
# VTA_OPCODE_GEMM 2
# VTA_OPCODE_FINISH 3
# VTA_OPCODE_ALU 4

def test_vta_fetch(nextlayer_lib, design_lib):
    # create a driver
    driver = FetchDriver(nextlayer_lib, design_lib)
    # reset accel for 10 cycles
    driver.reset()
    # write program
    for x in range(5):
        driver.write_instr_mem(x, x)
    # write instr pointer
    driver.write_instr_addr(0)
    # write instr count
    driver.write_instr_count(5)
    # launch
    driver.launch()
    # cycle counter
    cycles = 0
    # run accel, timeout set to 100000 cycles
    for i in range(1000):
        cycles += 1
        driver.run()
        if driver.is_finished():
            break
    # check results
    for x in range(5):
        load = driver.read_load_mem(x)
        gemm = driver.read_gemm_mem(x)
        store = driver.read_store_mem(x)
        print("addr:{} load:{} gemm:{} store:{}".format(x, load, gemm, store))
    # print cycles
    print("total cycles:{}".format(cycles))

if __name__ == "__main__":
    cur_dir = path.dirname(path.realpath(__file__))
    nextlayer_lib = path.join(cur_dir, "../target/release/libnextlayer.so")
    design_lib = path.join(cur_dir, "../hw/vta/fetch/xsim.dir/work.testbench/xsimk.so")
    sys.path.append(cur_dir)
    test_vta_fetch(nextlayer_lib, design_lib)
