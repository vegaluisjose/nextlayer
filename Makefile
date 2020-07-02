export LD_LIBRARY_PATH=$(XILINX_VIVADO)/lib/lnx64.o

default:
	make -C xsim
	ln -sf xsim/xsim.dir .
	cargo build --release
	python3 run.py

clean:
	make -C xsim clean
	rm xsim.dir
