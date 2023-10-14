compile:
	raco make knucleotide.racket-4.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../inputs/knucleotide-input25000000.txt" Racket k-nucleotide

run:
	racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/knucleotide.racket-4_racket.zo 0 < ../../inputs/knucleotide-input25000000.txt

clean:
	rm -rf compiled/