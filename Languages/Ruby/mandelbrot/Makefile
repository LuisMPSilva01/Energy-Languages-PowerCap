NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 mandelbrot.yarv-5.yarv 16000" Ruby mandelbrot $(NTIMES)

run:
	ruby -W0 mandelbrot.yarv-5.yarv 16000

mem:
	/usr/bin/time -v ruby -W0 mandelbrot.yarv-5.yarv 16000

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 mandelbrot.yarv-5.yarv 16000
