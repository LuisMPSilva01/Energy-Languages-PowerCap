compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe pidigits.dart-2.dart -o pidigits.dart-2.dart_run
xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "./pidigits.dart-2.dart_run 2000" Dart pidigits

run:
	./pidigits.dart-2.dart_run 2000

mem:
	/usr/bin/time -v  ./pidigits.dart-2.dart_run 2000

valgrind:
	valgrind --tool=massif --stacks=yes  ./pidigits.dart-2.dart_run 2000

clean:
	rm -r *.dart_run