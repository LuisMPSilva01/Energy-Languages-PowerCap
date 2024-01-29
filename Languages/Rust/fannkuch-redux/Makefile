NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 12" Rust fannkuch-redux $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 12

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v ./fannkuchredux.rust-3.rust_run 12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.rust-3.rust_run 12
