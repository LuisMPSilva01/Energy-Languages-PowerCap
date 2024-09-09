NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe binarytrees.dart-7.dart -o binarytrees.dart-7.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.dart-7.dart_run 21" Dart binary-trees $(NTIMES)

run:
	./binarytrees.dart-7.dart_run 21

mem:
	/usr/bin/time -v  ./binarytrees.dart-7.dart_run 21

valgrind:
	valgrind --tool=massif --stacks=yes  ./binarytrees.dart-7.dart_run 21

clean:
	rm -r *.dart_run