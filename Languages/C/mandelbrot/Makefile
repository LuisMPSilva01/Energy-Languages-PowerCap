NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -mno-fma -fno-finite-math-only -fopenmp mandelbrot.gcc-6.c -o mandelbrot.gcc-6.gcc_run 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.gcc-6.gcc_run 16000" C mandelbrot $(NTIMES)

run:
	./mandelbrot.gcc-6.gcc_run 16000

mem:
	/usr/bin/time -v ./mandelbrot.gcc-6.gcc_run 16000

clean:
	rm -f *.gcc_run
	rm -f output