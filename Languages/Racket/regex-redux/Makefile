compile:
	raco make regexredux.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/regexredux_racket.zo 0 < ../../regexredux-input5000000.txt" Racket regex-redux

run:
	racket ./compiled/regexredux_racket.zo 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v racket ./compiled/regexredux_racket.zo 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/regexredux_racket.zo 0 < ../../regexredux-input5000000.txt

clean:
	rm -rf compiled/