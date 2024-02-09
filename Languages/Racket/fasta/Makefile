NTIMES=20

export NTIMES

compile:
	/home/diguest/bin/raco make fasta.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/fasta.racket-3_racket.zo 25000000" Racket fasta $(NTIMES)

run:
	racket ./compiled/fasta.racket-3_racket.zo 25000000

mem:
	/usr/bin/time -v racket ./compiled/fasta.racket-3_racket.zo 25000000

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/fasta.racket-3_racket.zo 25000000

clean:
	rm -rf compiled/
