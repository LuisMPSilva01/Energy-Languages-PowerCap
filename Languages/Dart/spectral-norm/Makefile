NTIMES=20

export NTIMES

compile:
	/opt/src/dart-sdk/bin/dart analyze 
	/opt/src/dart-sdk/bin/dart compile exe spectralnorm.dart-6.dart -o spectralnorm.dart-6.dart_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.dart-6.dart_run 5500" Dart spectral-norm $(NTIMES)

run:
	./spectralnorm.dart-6.dart_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.dart-6.dart_run 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.dart-6.dart_run 5500

clean:
	rm -r *.dart_run