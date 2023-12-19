NTIMES=1

export NTIMES

compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe binarytrees.dart-7.dart -o binarytrees.dart-7.dart_run

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