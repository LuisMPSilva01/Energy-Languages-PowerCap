compile:
	cp -L nbody.node nbody.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node nbody.js 50000000" JavaScript n-body

run:
	node nbody.js 50000000

mem:
	/usr/bin/time -v  node nbody.js 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  node nbody.js 50000000
