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
	sudo ../../../RAPL/main "target/release/fannkuch-redux 12" Rust fannkuch-redux $(NTIMES)

run:
	target/release/fannkuch-redux 12

clean:
	rm -f -r target
	rm -f -r Cargo.lock