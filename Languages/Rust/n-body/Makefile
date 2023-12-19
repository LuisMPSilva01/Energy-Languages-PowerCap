NTIMES=1

export NTIMES

compile:
	cargo build	

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.rust-2.rust_run 50000000" Rust n-body $(NTIMES)

run:
	cargo run -- 50000000

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v ./nbody.rust-2.rust_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.rust-2.rust_run 50000000
