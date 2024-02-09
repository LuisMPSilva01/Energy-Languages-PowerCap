NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node mandelbrot.js 16000" JavaScript mandelbrot $(NTIMES)

run:
	node mandelbrot.js 16000

mem:
	/usr/bin/time -v node mandelbrot.js 16000

valgrind:
	valgrind --tool=massif --stacks=yes node mandelbrot.js 16000
