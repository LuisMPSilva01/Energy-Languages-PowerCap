NTIMES=20

export NTIMES

include ../../../config.env

ifndef CARGO_PATH
$(error CARGO_PATH is not set.)
endif

compile:
	$(CARGO_PATH) build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/spectral-norm 5500" Rust spectral-norm $(NTIMES)

run:
	target/release/spectral-norm 5500

clean:
	rm -f -r target
	rm -f -r Cargo.lock
