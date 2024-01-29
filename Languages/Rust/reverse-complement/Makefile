NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 0 < ../../../inputs/revcomp-input25000000.txt" Rust reverse-complement $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.rust-2.rust_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -r target
	rm -r Cargo.lock
	
valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.rust-2.rust_run 0 < ../../../inputs/revcomp-input25000000.txt
