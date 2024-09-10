NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe mandelbrot-dart-4.dart -o mandelbrot-dart-4.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot-dart-4.dart_run 16000" Dart mandelbrot $(NTIMES)

run:
	./mandelbrot-dart-4.dart_run 16000

mem:
	/usr/bin/time -v ./mandelbrot-dart-4.dart_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot-dart-4.dart_run 16000

clean:
	rm -r *.dart_run