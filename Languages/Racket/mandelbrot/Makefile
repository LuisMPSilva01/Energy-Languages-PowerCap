NTIMES=1

export NTIMES

compile:
	/home/diguest/bin/raco make mandelbrot.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/mandelbrot.racket-3_racket.zo 16000" Racket mandelbrot $(NTIMES)

run:
	racket ./compiled/mandelbrot.racket-3_racket.zo 16000

mem:
	/usr/bin/time -v racket ./compiled/mandelbrot.racket-3_racket.zo 16000

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/mandelbrot.racket-3_racket.zo 16000

clean:
	rm -rf compiled/
