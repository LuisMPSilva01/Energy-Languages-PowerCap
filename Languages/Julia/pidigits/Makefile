NTIMES=20

export NTIMES

compile:
	
measure:
	export JULIA_NUM_THREADS=4
	sudo ../../../RAPL/main "julia -O3  -- pidigits.julia-2.julia 10000" Julia pidigits $(NTIMES)
run:
	export JULIA_NUM_THREADS=4
	julia -O3  -- pidigits.julia-2.julia 10000
mem:
	export JULIA_NUM_THREADS=4
	/usr/bin/time -v  julia -O3  -- pidigits.julia-2.julia 10000
valmem:
	export JULIA_NUM_THREADS=4
	valgrind --tool=massif --stacks=yes  julia -O3  -- pidigits.julia-2.julia 10000
