NTIMES=1

export NTIMES

compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe fasta.dart -o fasta.dart_run

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