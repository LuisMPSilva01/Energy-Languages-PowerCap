compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "perl knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt" Perl k-nucleotide

run:
	perl knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v perl knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes perl knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt
