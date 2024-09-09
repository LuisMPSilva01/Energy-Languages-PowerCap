NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) fannkuchredux.node-4.js 12" JavaScript fannkuch-redux $(NTIMES)

run:
	node fannkuchredux.node-4.js 12

mem:
	/usr/bin/time -v node fannkuchredux.node-4.js 12

valgrind:
	valgrind --tool=massif --stacks=yes node fannkuchredux.node-4.js 12
