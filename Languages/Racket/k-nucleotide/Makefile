NTIMES=20

export NTIMES

compile:
	/home/diguest/bin/raco make knucleotide.racket-4.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../../inputs/knucleotide-input25000000.txt" Racket k-nucleotide $(NTIMES)

run:
	racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -rf compiled/
