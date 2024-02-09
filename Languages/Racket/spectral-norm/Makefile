NTIMES=20

export NTIMES

compile:
	/home/diguest/bin/raco make spectralnorm.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/spectralnorm.racket-3_racket.zo 5500" Racket spectral-norm $(NTIMES)

run:
	racket ./compiled/spectralnorm.racket-3_racket.zo 5500

mem:
	/usr/bin/time -v racket ./compiled/spectralnorm.racket-3_racket.zo 5500

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/spectralnorm.racket-3_racket.zo 5500

clean:
	rm -rf compiled/
