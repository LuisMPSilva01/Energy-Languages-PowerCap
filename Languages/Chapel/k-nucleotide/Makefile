compile:
	/opt/src/chapel-1.29.0/bin/chpl --fast knucleotide.chapel-4.chpl -o knucleotide.chapel-4.chapel_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./knucleotide.chapel-4.chapel_run --n=0 < knucleotide-input25000000.txt" Chapel k-nucleotide

run:
	./knucleotide.chapel-4.chapel_run --n=0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.chapel-4.chapel_run --n=0 < knucleotide-input25000000.txt


valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.chapel-4.chapel_run --n=0 < knucleotide-input25000000.txt

