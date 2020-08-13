export LD_LIBRARY_PATH=$(XILINX_VIVADO)/lib/lnx64.o

HW_DIR = $(abspath .)/hw

default: test_add test_vadd

test_add:
	make -C $(HW_DIR)/add
	ln -sf $(HW_DIR)/add/xsim.dir .
	cargo build --release
	python3 python/test_add.py

test_vadd:
	make -C $(HW_DIR)/vadd
	ln -sf $(HW_DIR)/vadd/xsim.dir .
	cargo build --release
	python3 python/test_vadd.py

clean:
	make -C $(HW_DIR)/add clean
	make -C $(HW_DIR)/vadd clean
	rm xsim.dir

.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy
	cargo build
