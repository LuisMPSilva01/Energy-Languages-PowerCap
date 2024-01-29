NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 50000000" Rust n-body $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 50000000

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v ./nbody.rust-2.rust_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.rust-2.rust_run 50000000
