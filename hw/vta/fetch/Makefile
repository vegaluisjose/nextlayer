default: build_vta xvlog xelab

VTA_DIR = $(abspath ../../../)/3rdparty/incubator-tvm-vta
VTA_HARDWARE_XILINX = $(VTA_DIR)/hardware/xilinx
VTA_FETCH_DIR = $(VTA_DIR)/build/hardware/xilinx/hls/sim_1x16_i8w8a32_15_15_18_17/vta_fetch/soln/impl/verilog

SRC_DIR = $(abspath .)/src
FILES = \
$(VTA_FETCH_DIR)/fetch_CONTROL_BUS_s_axi.v \
$(VTA_FETCH_DIR)/fetch_ins_port_m_axi.v \
$(VTA_FETCH_DIR)/regslice_core.v \
$(VTA_FETCH_DIR)/fetch.v \
$(SRC_DIR)/axi_ram.v \
$(SRC_DIR)/simple_dpram_sclk.v \
$(SRC_DIR)/fifo.v \
$(SRC_DIR)/top.v \
$(SRC_DIR)/testbench.v

build_vta:
	make -C $(VTA_HARDWARE_XILINX) ip

xvlog:
	xvlog --sv --incr --relax $(FILES)

xelab:
	xelab --timescale 1ps/1ps -O3 -dll -L xpm testbench

clean:
	rm -rf xsim.dir *.log *.pb *.so *.jou *.wdb sim *.o
