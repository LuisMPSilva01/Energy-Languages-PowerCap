NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt" JavaScript reverse-complement $(NTIMES)

run:
	node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt
