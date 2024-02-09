NTIMES=20

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/binary-trees 21" Rust binary-trees $(NTIMES)

run:
	target/release/binary-trees 21

clean:
	rm -f -r target
	rm -f -r Cargo.lock
