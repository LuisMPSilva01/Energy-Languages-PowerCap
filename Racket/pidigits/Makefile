compile:
	raco make pidigits.racket-2.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/pidigits.racket-2_racket.zo 10000" Racket pidigits

run:
	racket ./compiled/pidigits.racket-2_racket.zo 10000

mem:
	/usr/bin/time -v racket ./compiled/pidigits.racket-2_racket.zo 10000

valgrind:
	valgrind --tool=massif --stacks=yes  racket ./compiled/pidigits.racket-2_racket.zo 10000

clean:
	rm -rf compiled/
