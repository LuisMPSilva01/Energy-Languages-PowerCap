NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/n-body 50000000" Rust n-body $(NTIMES)

run:
	target/release/n-body 50000000

clean:
	rm -f -r target
	rm -f -r Cargo.lock
