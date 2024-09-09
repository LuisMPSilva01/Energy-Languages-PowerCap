NTIMES=20

export NTIMES

include ../../../config.env

ifndef DART_PATH
$(error DART_PATH is not set.)
endif

compile:
	$(DART_PATH) analyze 
	$(DART_PATH) compile exe spectralnorm.dart-6.dart -o spectralnorm.dart-6.dart_run

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