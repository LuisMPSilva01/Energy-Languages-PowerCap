NTIMES=20

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/mandelbrot 16000" Rust mandelbrot $(NTIMES)

run:
	target/release/mandelbrot 16000

clean:
	rm -f -r target
	rm -f -r Cargo.lock
