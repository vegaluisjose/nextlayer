export LD_LIBRARY_PATH=$(XILINX_VIVADO)/lib/lnx64.o

TARGET = vadd
TARGET_DIR = $(abspath .)/designs/$(TARGET)

default:
	make -C $(TARGET_DIR)
	ln -sf $(TARGET_DIR)/xsim.dir .
	cargo build --release
	python3 python/$(TARGET).py

clean:
	make -C $(TARGET_DIR) clean
	rm xsim.dir
