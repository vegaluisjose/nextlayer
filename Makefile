export LD_LIBRARY_PATH=$(XILINX_VIVADO)/lib/lnx64.o

DESIGN_DIR = $(abspath .)/designs

default: test_add test_vadd

test_add:
	make -C $(DESIGN_DIR)/add
	ln -sf $(DESIGN_DIR)/add/xsim.dir .
	cargo build --release
	python3 python/test_add.py

test_vadd:
	make -C $(DESIGN_DIR)/vadd
	ln -sf $(DESIGN_DIR)/vadd/xsim.dir .
	cargo build --release
	python3 python/test_vadd.py

clean:
	make -C $(DESIGN_DIR)/add clean
	make -C $(DESIGN_DIR)/vadd clean
	rm xsim.dir

.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy
	cargo build
