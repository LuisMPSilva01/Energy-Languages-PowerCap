NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) mandelbrot.perl 16000" Perl mandelbrot $(NTIMES)

run:
	$(PERL_PATH) mandelbrot.perl 16000

mem:
	/usr/bin/time -v $(PERL_PATH) mandelbrot.perl 16000

valgrind:
	valgrind --tool=massif --stacks=yes $(PERL_PATH) mandelbrot.perl 16000
