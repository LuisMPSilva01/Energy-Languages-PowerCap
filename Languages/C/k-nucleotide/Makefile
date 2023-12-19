NTIMES=1

export NTIMES

compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp -IInclude knucleotide.c -o knucleotide.gcc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.gcc_run 0 < ../../../inputs/knucleotide-input25000000.txt" C k-nucleotide $(NTIMES)

run:
	./knucleotide.gcc_run 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.gcc_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.gcc_run