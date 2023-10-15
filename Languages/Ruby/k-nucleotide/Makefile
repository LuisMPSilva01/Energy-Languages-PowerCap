compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 knucleotide.yarv-7.yarv 0 < ../../../inputs/knucleotide-input25000000.txt" Ruby k-nucleotide

run:
	ruby -W0 knucleotide.yarv-7.yarv 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ruby -W0 knucleotide.yarv-7.yarv 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 knucleotide.yarv-7.yarv 0 < ../../../inputs/knucleotide-input25000000.txt
