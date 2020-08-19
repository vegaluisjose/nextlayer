default: xvlog xelab

SRC_DIR = $(abspath .)/src
FILES = \
$(SRC_DIR)/simple_dpram_sclk.v \
$(SRC_DIR)/fifo.v \
$(SRC_DIR)/top.v \
$(SRC_DIR)/testbench.v

xvlog:
	xvlog --sv --incr --relax $(FILES)

xelab:
	xelab --timescale 1ps/1ps -O3 -dll -L xpm testbench

clean:
	rm -rf xsim.dir *.log *.pb *.so *.jou *.wdb sim *.o
