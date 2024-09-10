NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -pthread fannkuchredux.gcc-6.c -o fannkuchredux.gcc-6.gcc_run 
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.gcc-6.gcc_run 12" C fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.gcc-6.gcc_run 12

mem:
	 /usr/bin/time -v ./fannkuchredux.gcc-6.gcc_run 12


clean:
	rm -f *.gcc_run