NTIMES=1

export NTIMES

compile:
	/home/diguest/bin/raco make regexredux.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt" Racket regex-redux $(NTIMES)

run:
	racket ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v racket ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -rf compiled/
