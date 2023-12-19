NTIMES=1

export NTIMES

compile:
	cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.rust-3.rust_run 12" Rust fannkuch-redux $(NTIMES)

run:
	cargo run -- 12

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v ./fannkuchredux.rust-3.rust_run 12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.rust-3.rust_run 12
