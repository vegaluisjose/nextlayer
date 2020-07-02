export LD_LIBRARY_PATH=$(XILINX_VIVADO)/lib/lnx64.o

default:
	make -C xsim
	ln -sf xsim/xsim.dir .
	cargo run

clean:
	make -C xsim clean
	rm xsim.dir
