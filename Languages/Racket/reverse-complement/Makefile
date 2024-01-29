NTIMES=1

export NTIMES

compile:
	/home/diguest/bin/raco make revcomp.racket-2.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt" Racket reverse-complement $(NTIMES)

run:
	racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -rf compiled/
