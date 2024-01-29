NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 0 < ../../../inputs/knucleotide-input25000000.txt" Rust k-nucleotide $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -r target
	rm -r Cargo.lock
	
mem:
	/usr/bin/time -v ./knucleotide.rust-4.rust_run 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.rust-4.rust_run 0 < ../../../inputs/knucleotide-input25000000.txt
