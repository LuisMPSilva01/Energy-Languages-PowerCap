NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe nbody.dart-6.dart -o nbody.dart-6.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.dart-6.dart_run 50000000" Dart n-body $(NTIMES)

run:
	./nbody.dart-6.dart_run 50000000

mem:
	/usr/bin/time -v ./nbody.dart-6.dart_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.dart-6.dart_run 50000000

clean:
	rm -r *.dart_run