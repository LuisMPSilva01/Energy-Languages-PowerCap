NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node nbody.js 50000000" JavaScript n-body $(NTIMES)

run:
	node nbody.js 50000000

mem:
	/usr/bin/time -v  node nbody.js 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  node nbody.js 50000000
