NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build --release

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "target/release/regex-redux 0 < ../../../inputs/regexredux-input5000000.txt" Rust regex-redux $(NTIMES)

run:
	target/release/regex-redux 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f -r target
	rm -f -r Cargo.lock
