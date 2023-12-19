NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "node regexredux.node-2.js 0 < ../../../inputs/regexredux-input5000000.txt" JavaScript regex-redux $(NTIMES)

run:
	node regexredux.node-2.js 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v node regexredux.node-2.js 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node regexredux.node-2.js 0 < ../../../inputs/regexredux-input5000000.txt
