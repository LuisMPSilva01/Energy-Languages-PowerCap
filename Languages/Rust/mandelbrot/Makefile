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
	sudo ../../../RAPL/main "target/release/mandelbrot 16000" Rust mandelbrot $(NTIMES)

run:
	target/release/mandelbrot 16000

clean:
	rm -f -r target
	rm -f -r Cargo.lock
