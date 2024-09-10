NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp fasta.gcc-9.c -o fasta.gcc-9.gcc_run 
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.gcc-9.gcc_run 25000000" C fasta $(NTIMES)

run:
	./fasta.gcc-9.gcc_run 25000000

mem:
	/usr/bin/time -v ./fasta.gcc-9.gcc_run 25000000

clean:
	rm -f *.gcc_run