NTIMES=1

export NTIMES

compile:
	cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.rust-4.rust_run 5500" Rust spectral-norm $(NTIMES)

run:
	cargo run -- 5500

mem:
	/usr/bin/time -v ./spectralnorm.rust-4.rust_run 5500

clean:
	rm -r target
	rm -r Cargo.lock
	
valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.rust-4.rust_run 5500
