NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/spectral-norm 5500" Rust spectral-norm $(NTIMES)

run:
	target/release/spectral-norm 5500

clean:
	rm -f -r target
	rm -f -r Cargo.lock
