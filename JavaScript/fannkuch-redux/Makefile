compile:
	cp -L fannkuchredux.node-4.node fannkuchredux.node-4.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node fannkuchredux.node-4.js 12" JavaScript fannkuch-redux

run:
	node fannkuchredux.node-4.js 12

mem:
	/usr/bin/time -v node fannkuchredux.node-4.js 12

valgrind:
	valgrind --tool=massif --stacks=yes node fannkuchredux.node-4.js 12
