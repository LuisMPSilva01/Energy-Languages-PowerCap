NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 25000000" Rust fasta $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 25000000

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v ./fasta.rust-2.rust_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.rust-2.rust_run 25000000
