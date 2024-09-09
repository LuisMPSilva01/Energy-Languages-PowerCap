NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) mandelbrot.js 16000" JavaScript mandelbrot $(NTIMES)

run:
	node mandelbrot.js 16000

mem:
	/usr/bin/time -v node mandelbrot.js 16000

valgrind:
	valgrind --tool=massif --stacks=yes node mandelbrot.js 16000
