NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) pidigits.js 10000" JavaScript pidigits $(NTIMES)

run:
	node pidigits.js 10000

mem:
	/usr/bin/time -v  node pidigits.js 10000

valgrind:
	valgrind --tool=massif --stacks=yes  node pidigits.js 10000
