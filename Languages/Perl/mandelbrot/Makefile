NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "perl mandelbrot.perl 16000" Perl mandelbrot $(NTIMES)

run:
	perl mandelbrot.perl 16000

mem:
	/usr/bin/time -v perl mandelbrot.perl 16000

valgrind:
	valgrind --tool=massif --stacks=yes perl mandelbrot.perl 16000
