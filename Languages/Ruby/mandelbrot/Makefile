compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 mandelbrot.yarv-5.yarv 16000" Ruby mandelbrot

run:
	ruby -W0 mandelbrot.yarv-5.yarv 16000

mem:
	/usr/bin/time -v ruby -W0 mandelbrot.yarv-5.yarv 16000

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 mandelbrot.yarv-5.yarv 16000
