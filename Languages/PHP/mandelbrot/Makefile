NTIMES=20

export NTIMES

include ../../../config.env

ifndef PHP_PATH
$(error PHP_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PHP_PATH)  mandelbrot.php-3.php 16000" PHP mandelbrot $(NTIMES)

run:
	$(PHP_PATH)  mandelbrot.php-3.php 16000

mem:
	/usr/bin/time -v $(PHP_PATH)  mandelbrot.php-3.php 16000

valgrind:
	valgrind --tool=massif --stacks=yes $(PHP_PATH)  mandelbrot.php-3.php 16000
