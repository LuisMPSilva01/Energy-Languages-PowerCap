NTIMES=1

export NTIMES

compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe fannkuchredux.dart-5.dart -o fannkuchredux.dart-5.dart_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.dart-5.dart_run 12" Dart fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.dart-5.dart_run 12

mem:
	/usr/bin/time -v ./fannkuchredux.dart-5.dart_run 12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.dart-5.dart_run 12

clean:
	rm -r *.dart_run