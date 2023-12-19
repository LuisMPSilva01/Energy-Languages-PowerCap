NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "node spectralnorm.node-2.js 5500" JavaScript spectral-norm $(NTIMES)

run:
	node spectralnorm.node-2.js 5500

mem:
	/usr/bin/time -v node spectralnorm.node-2.js 5500

valgrind:
	valgrind --tool=massif --stacks=yes node spectralnorm.node-2.js 5500
