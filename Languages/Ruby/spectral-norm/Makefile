compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 spectralnorm.yarv-5.yarv 5500" Ruby spectral-norm

run:
	ruby -W0 spectralnorm.yarv-5.yarv 5500

mem:
	/usr/bin/time -v ruby -W0 spectralnorm.yarv-5.yarv 5500

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 spectralnorm.yarv-5.yarv 5500

