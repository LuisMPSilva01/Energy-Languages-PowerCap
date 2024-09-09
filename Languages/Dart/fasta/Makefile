NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe fasta.dart -o fasta.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.dart_run 25000000" Dart fasta $(NTIMES)

run:
	./fasta.dart_run 25000000

mem:
	/usr/bin/time -v  ./fasta.dart_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./fasta.dart_run 25000000

clean:
	rm -r *.dart_run