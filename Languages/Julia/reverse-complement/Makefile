compile:
	
measure:
	export JULIA_NUM_THREADS=4
	sudo ../../RAPL/main "julia -O3  -- revcomp.julia-8.julia 0 < ../../revcomp-input25000000.txt" Julia reverse-complement
run:
	export JULIA_NUM_THREADS=4
	julia -O3  -- revcomp.julia-8.julia 0 < ../../revcomp-input25000000.txt
mem:
	export JULIA_NUM_THREADS=4
	/usr/bin/time -v  julia  -O3  -- revcomp.julia-8.julia 0 < ../../revcomp-input25000000.txt
valmem:
	export JULIA_NUM_THREADS=4
	valgrind --tool=massif --stacks=yes  julia  -O3  -- revcomp.julia-8.julia 0 < ../../revcomp-input25000000.txt
