NTIMES=20

export NTIMES

include ../../../config.env

ifndef JULIA_PATH
$(error JULIA_PATH is not set.)
endif

compile:
	
measure:
	export JULIA_NUM_THREADS=4
	sudo ../../../RAPL/main "$(JULIA_PATH) -O3  -- pidigits.julia-2.julia 10000" Julia pidigits $(NTIMES)
run:
	export JULIA_NUM_THREADS=4
	$(JULIA_PATH) -O3  -- pidigits.julia-2.julia 10000
mem:
	export JULIA_NUM_THREADS=4
	/usr/bin/time -v  $(JULIA_PATH) -O3  -- pidigits.julia-2.julia 10000
valmem:
	export JULIA_NUM_THREADS=4
	valgrind --tool=massif --stacks=yes  $(JULIA_PATH) -O3  -- pidigits.julia-2.julia 10000
