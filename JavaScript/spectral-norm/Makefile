compile:
	cp -L spectralnorm.node-2.node spectralnorm.node-2.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node spectralnorm.node-2.js 5500" JavaScript spectral-norm

run:
	node spectralnorm.node-2.js 5500

mem:
	/usr/bin/time -v node spectralnorm.node-2.js 5500

valgrind:
	valgrind --tool=massif --stacks=yes node spectralnorm.node-2.js 5500
