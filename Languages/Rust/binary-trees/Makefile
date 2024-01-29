NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 21" Rust binary-trees $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 21

mem:
	/usr/bin/time -v ./binarytrees.rust-2.rust_run 21

clean:
	rm -r target
	rm -r Cargo.lock
	
valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.rust-2.rust_run 21 
