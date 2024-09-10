NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) binarytrees.js 21" JavaScript binary-trees $(NTIMES)

run:
	node binarytrees.js 21

mem:
	/usr/bin/time -v node binarytrees.js 21

valgrind:
	valgrind --tool=massif --stacks=yes node binarytrees.js 21
