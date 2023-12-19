NTIMES=1

export NTIMES

compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe nbody.dart-6.dart -o nbody.dart-6.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.dart-6.dart_run 50000000" Dart n-body $(NTIMES)

run:
	./nbody.dart-6.dart_run 50000000

mem:
	/usr/bin/time -v ./nbody.dart-6.dart_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.dart-6.dart_run 50000000

clean:
	rm -r *.dart_run