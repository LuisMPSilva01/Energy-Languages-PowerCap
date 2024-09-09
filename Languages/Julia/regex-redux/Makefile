NTIMES=20

export NTIMES

include ../../../config.env

ifndef JULIA_PATH
$(error JULIA_PATH is not set.)
endif

compile:
	
measure:
	export JULIA_NUM_THREADS=4
	sudo ../../../RAPL/main "$(JULIA_PATH) -O3  -- regexredux.julia-3.julia 0 < ../../../inputs/regexredux-input5000000.txt" Julia regex-redux $(NTIMES)
run:
	export JULIA_NUM_THREADS=4
	$(JULIA_PATH) -O3  -- regexredux.julia-3.julia 0 < ../../../inputs/regexredux-input5000000.txt
mem:
	export JULIA_NUM_THREADS=4
	/usr/bin/time -v  $(JULIA_PATH) -O3  -- regexredux.julia-3.julia 0 < ../../../inputs/regexredux-input5000000.txt
valmem:
	export JULIA_NUM_THREADS=4
	valgrind --tool=massif --stacks=yes  $(JULIA_PATH) -O3  -- regexredux.julia-3.julia 0 < ../../../inputs/regexredux-input5000000.txt
