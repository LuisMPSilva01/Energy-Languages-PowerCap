NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe revcomp.dart-5.dart -o revcomp.dart-5.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt" Dart reverse-complement $(NTIMES)

run:
	./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -r *.dart_run