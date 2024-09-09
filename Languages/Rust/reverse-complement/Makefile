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
	sudo ../../../RAPL/main "target/release/reverse-complement 0 < ../../../inputs/revcomp-input25000000.txt" Rust reverse-complement $(NTIMES)

run:
	target/release/reverse-complement 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f -r target
	rm -f -r Cargo.lock
