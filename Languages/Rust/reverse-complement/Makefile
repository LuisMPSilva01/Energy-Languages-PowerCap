compile:
	rustc -C opt-level=3 -C target-cpu=core2 -C lto -L /usr/local/src/rust-libs revcomp.rs -o revcomp.rust-2.rust_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./revcomp.rust-2.rust_run 0 < ../../../inputs/revcomp-input25000000.txt" Rust reverse-complement

run:
	./revcomp.rust-2.rust_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.rust-2.rust_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.rust-2.rust_run 0 < ../../../inputs/revcomp-input25000000.txt
