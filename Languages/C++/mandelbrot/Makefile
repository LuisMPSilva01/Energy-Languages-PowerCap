NTIMES=20

export NTIMES

include ../../../config.env

ifndef GPP_PATH
$(error GPP_PATH is not set.)
endif

compile:
	$(GPP_PATH) -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -fopenmp mandelbrot.c++ -o mandelbrot.c++.o &&  $(GPP_PATH) mandelbrot.c++.o -o mandelbrot.gpp_run -fopenmp 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.gpp_run 16000" C++ mandelbrot $(NTIMES)

run:
	./mandelbrot.gpp_run 16000

mem:
	/usr/bin/time -v ./mandelbrot.gpp_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.gpp_run 16000

clean:
	rm -f *.gpp_run
	rm -f *.o