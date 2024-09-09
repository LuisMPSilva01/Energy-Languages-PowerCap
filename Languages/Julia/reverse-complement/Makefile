NTIMES=20

export NTIMES

include ../../../config.env

ifndef JULIA_PATH
$(error JULIA_PATH is not set.)
endif

compile:
	
measure:
	export JULIA_NUM_THREADS=4
	sudo ../../../RAPL/main "$(JULIA_PATH) -O3  -- revcomp.julia-8.julia 0 < ../../../inputs/revcomp-input25000000.txt" Julia reverse-complement $(NTIMES)
run:
	export JULIA_NUM_THREADS=4
	$(JULIA_PATH) -O3  -- revcomp.julia-8.julia 0 < ../../../inputs/revcomp-input25000000.txt
mem:
	export JULIA_NUM_THREADS=4
	/usr/bin/time -v  julia  -O3  -- revcomp.julia-8.julia 0 < ../../../inputs/revcomp-input25000000.txt
valmem:
	export JULIA_NUM_THREADS=4
	valgrind --tool=massif --stacks=yes  julia  -O3  -- revcomp.julia-8.julia 0 < ../../../inputs/revcomp-input25000000.txt
