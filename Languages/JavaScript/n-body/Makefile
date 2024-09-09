NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) nbody.js 50000000" JavaScript n-body $(NTIMES)

run:
	node nbody.js 50000000

mem:
	/usr/bin/time -v  node nbody.js 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  node nbody.js 50000000
