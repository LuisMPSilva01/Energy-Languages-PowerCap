NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "node binarytrees.js 21" JavaScript binary-trees $(NTIMES)

run:
	node binarytrees.js 21

mem:
	/usr/bin/time -v node binarytrees.js 21

valgrind:
	valgrind --tool=massif --stacks=yes node binarytrees.js 21