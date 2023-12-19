NTIMES=1

export NTIMES

compile:
	cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.rust-4.rust_run 16000" Rust mandelbrot $(NTIMES)

run:
	cargo run -- 16000

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v  ./mandelbrot.rust-4.rust_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes  ./mandelbrot.rust-4.rust_run 16000
