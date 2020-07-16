default: xvlog xelab

SRC_DIR = $(abspath .)/src
FILES = \
$(SRC_DIR)/axi_ram.v \
$(SRC_DIR)/krnl_vadd_rtl_adder.sv \
$(SRC_DIR)/krnl_vadd_rtl_axi_read_master.sv \
$(SRC_DIR)/krnl_vadd_rtl_axi_write_master.sv \
$(SRC_DIR)/krnl_vadd_rtl_control_s_axi.v \
$(SRC_DIR)/krnl_vadd_rtl_counter.sv \
$(SRC_DIR)/krnl_vadd_rtl_int.sv \
$(SRC_DIR)/krnl_vadd_rtl.v \
$(SRC_DIR)/top.v \
$(SRC_DIR)/testbench.v

xvlog:
	xvlog --sv --incr --relax $(FILES)

xelab:
	xelab --timescale 1ps/1ps -O3 -dll -L xpm testbench

clean:
	rm -rf xsim.dir *.log *.pb *.so *.jou *.wdb sim *.o
