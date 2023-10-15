compile:
	cp -L knucleotide.node-2.node knucleotide.node-2.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node knucleotide.node-2.js 0 < ../../../inputs/knucleotide-input25000000.txt" JavaScript k-nucleotide

run:
	node knucleotide.node-2.js 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v node knucleotide.node-2.js 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node knucleotide.node-2.js 0 < ../../../inputs/knucleotide-input25000000.txt
