compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe regexredux.dart-5.dart -o regexredux.dart-5.dart_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.dart-5.dart_run 0 < ../../inputs/regexredux-input5000000.txt" Dart regex-redux

run:
	./regexredux.dart-5.dart_run 0 < ../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  ./regexredux.dart-5.dart_run 0 < ../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  regexredux.dart-5.dart_run 0 < ../../inputs/regexredux-input5000000.txt

clean:
	rm -r *.dart_run