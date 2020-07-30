export LD_LIBRARY_PATH=$(XILINX_VIVADO)/lib/lnx64.o

TARGET = vadd
DESIGN_DIR = $(abspath .)/designs
TARGET_DIR = $(DESIGN_DIR)/$(TARGET)

default:
	make -C $(DESIGN_DIR)/add
	ln -sf $(DESIGN_DIR)/add/xsim.dir .
	cargo build --release
	python3 python/test_add.py

clean:
	make -C $(TARGET_DIR) clean
	rm xsim.dir

.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy
	cargo build
