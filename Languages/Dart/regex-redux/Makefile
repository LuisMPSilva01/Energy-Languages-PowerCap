NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe regexredux.dart-5.dart -o regexredux.dart-5.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.dart-5.dart_run 0 < ../../../inputs/regexredux-input5000000.txt" Dart regex-redux $(NTIMES)

run:
	./regexredux.dart-5.dart_run 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  ./regexredux.dart-5.dart_run 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  regexredux.dart-5.dart_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -r *.dart_run