NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp -I/klib knucleotide.c -o knucleotide.gcc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.gcc_run 0 < ../../../inputs/knucleotide-input25000000.txt" C k-nucleotide $(NTIMES)

run:
	./knucleotide.gcc_run 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.gcc_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.gcc_run