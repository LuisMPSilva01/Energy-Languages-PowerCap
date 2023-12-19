NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "php -n  spectralnorm.php 5500" PHP spectral-norm $(NTIMES)

run:
	php -n  spectralnorm.php 5500

mem:
	/usr/bin/time -v php -n  spectralnorm.php 5500

valgrind:
	valgrind --tool=massif --stacks=yes php -n  spectralnorm.php 5500
