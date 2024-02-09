NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node binarytrees.js 21" JavaScript binary-trees $(NTIMES)

run:
	node binarytrees.js 21

mem:
	/usr/bin/time -v node binarytrees.js 21

valgrind:
	valgrind --tool=massif --stacks=yes node binarytrees.js 21
