NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -pthread revcomp.gcc-7.c -o revcomp.gcc-7.gcc_run 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.gcc-7.gcc_run 0 < ../../../inputs/revcomp-input25000000.txt" C reverse-complement $(NTIMES)

run:
	./revcomp.gcc-7.gcc_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.gcc_run