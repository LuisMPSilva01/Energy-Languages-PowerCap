compile:
	cp -L pidigits.node pidigits.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node pidigits.js 10000" JavaScript pidigits

run:
	node pidigits.js 10000

mem:
	/usr/bin/time -v  node pidigits.js 10000

valgrind:
	valgrind --tool=massif --stacks=yes  node pidigits.js 10000
