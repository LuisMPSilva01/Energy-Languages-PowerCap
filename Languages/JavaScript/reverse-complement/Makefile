compile:
	cp -L revcomp.node-7.node revcomp.node-7.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node revcomp.node-7.js 0 < ../../revcomp-input25000000.txt" JavaScript reverse-complement

run:
	node revcomp.node-7.js 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v node revcomp.node-7.js 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node revcomp.node-7.js 0 < ../../revcomp-input25000000.txt
