NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node pidigits.js 10000" JavaScript pidigits $(NTIMES)

run:
	node pidigits.js 10000

mem:
	/usr/bin/time -v  node pidigits.js 10000

valgrind:
	valgrind --tool=massif --stacks=yes  node pidigits.js 10000
