default: xvlog xelab

xvlog:
	xvlog --sv --incr --relax testbench.v

xelab:
	xelab --timescale 1ps/1ps -O3 -dll testbench

clean:
	rm -rf xsim.dir *.log *.pb *.so *.jou *.wdb sim *.o
