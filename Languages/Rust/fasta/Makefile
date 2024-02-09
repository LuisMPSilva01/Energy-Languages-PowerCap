NTIMES=20

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/fasta 25000000" Rust fasta $(NTIMES)

run:
	target/release/fasta 25000000

clean:
	rm -f -r target
	rm -f -r Cargo.lock
