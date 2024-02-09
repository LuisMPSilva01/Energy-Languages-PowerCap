NTIMES=20

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/k-nucleotide 0 < ../../../inputs/knucleotide-input25000000.txt" Rust k-nucleotide $(NTIMES)

run:
	target/release/k-nucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f -r target
	rm -f -r Cargo.lock