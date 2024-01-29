NTIMES=1

export NTIMES

compile:
	/home/diguest/bin/raco make fannkuchredux.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/fannkuchredux.racket-3_racket.zo 12" Racket fannkuch-redux $(NTIMES)

run:
	racket ./compiled/fannkuchredux.racket-3_racket.zo 12

mem:
	/usr/bin/time -v racket ./compiled/fannkuchredux.racket-3_racket.zo 12

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/fannkuchredux.racket-3_racket.zo 12

clean:
	rm -rf compiled/
