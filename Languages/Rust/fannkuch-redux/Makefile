NTIMES=20

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/fannkuch-redux 12" Rust fannkuch-redux $(NTIMES)

run:
	target/release/fannkuch-redux 12

clean:
	rm -f -r target
	rm -f -r Cargo.lock