compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe knucleotide.dart-3.dart -o knucleotide.dart-3.dart_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./knucleotide.dart-3.dart_run 0 < ../../inputs/knucleotide-input25000000.txt" Dart k-nucleotide

run:
	./knucleotide.dart-3.dart_run 0 < ../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v  ./knucleotide.dart-3.dart_run 0 < ../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./knucleotide.dart-3.dart_run 0 < ../../inputs/knucleotide-input25000000.txt

clean:
	rm -r *.dart_run