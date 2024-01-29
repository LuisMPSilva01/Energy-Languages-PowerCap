NTIMES=1

export NTIMES

compile:
	/home/diguest/.cargo/bin/cargo build

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.cargo/bin/cargo run -- 0 < ../../../inputs/regexredux-input5000000.txt" Rust regex-redux $(NTIMES)

run:
	/home/diguest/.cargo/bin/cargo run -- 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.rust_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -r target
	rm -r Cargo.lock
	
valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.rust_run 0 < ../../../inputs/regexredux-input5000000.txt
