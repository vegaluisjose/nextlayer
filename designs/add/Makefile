default: xvlog xelab

SRC_DIR = $(abspath .)/src
FILES = \
$(SRC_DIR)/testbench.v \
$(SRC_DIR)/adder.v

xvlog:
	xvlog --sv --incr --relax $(FILES)

xelab:
	xelab --timescale 1ps/1ps -O3 -dll testbench

clean:
	rm -rf xsim.dir *.log *.pb *.so *.jou *.wdb sim *.o
