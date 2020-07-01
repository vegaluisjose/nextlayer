default:
	make -C lib
	cargo build --release
	python3 run.py
