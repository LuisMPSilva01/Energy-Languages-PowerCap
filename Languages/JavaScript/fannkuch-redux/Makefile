NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node fannkuchredux.node-4.js 12" JavaScript fannkuch-redux $(NTIMES)

run:
	node fannkuchredux.node-4.js 12

mem:
	/usr/bin/time -v node fannkuchredux.node-4.js 12

valgrind:
	valgrind --tool=massif --stacks=yes node fannkuchredux.node-4.js 12
