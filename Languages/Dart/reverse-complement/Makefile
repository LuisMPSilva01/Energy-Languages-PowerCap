compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe revcomp.dart-5.dart -o revcomp.dart-5.dart_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt" Dart reverse-complement

run:
	./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.dart-5.dart_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -r *.dart_run