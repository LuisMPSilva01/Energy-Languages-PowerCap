NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 16000" Rust mandelbrot $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 16000

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v  ./mandelbrot.rust-4.rust_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes  ./mandelbrot.rust-4.rust_run 16000
