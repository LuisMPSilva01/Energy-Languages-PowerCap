compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast mandelbrot.chapel-3.chpl -o mandelbrot.chapel-3.chapel_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./mandelbrot.chapel-3.chapel_run --n=16000" Chapel mandelbrot

run:
	./mandelbrot.chapel-3.chapel_run --n=16000

mem:
	/usr/bin/time -v ./mandelbrot.chapel-3.chapel_run --n=16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.chapel-3.chapel_run --n=16000
