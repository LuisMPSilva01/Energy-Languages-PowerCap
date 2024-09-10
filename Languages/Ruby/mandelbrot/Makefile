NTIMES=20

export NTIMES

include ../../../config.env

ifndef RUBY_PATH
$(error RUBY_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RUBY_PATH) -W0 mandelbrot.yarv-5.yarv 16000" Ruby mandelbrot $(NTIMES)

run:
	$(RUBY_PATH) -W0 mandelbrot.yarv-5.yarv 16000

mem:
	/usr/bin/time -v $(RUBY_PATH) -W0 mandelbrot.yarv-5.yarv 16000

valgrind:
	valgrind --tool=massif --stacks=yes $(RUBY_PATH) -W0 mandelbrot.yarv-5.yarv 16000
