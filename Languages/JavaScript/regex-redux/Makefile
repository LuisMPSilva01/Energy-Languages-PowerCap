compile:
	cp -L regexredux.node-2.node regexredux.node-2.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node regexredux.node-2.js 0 < ../../regexredux-input5000000.txt" JavaScript regex-redux

run:
	node regexredux.node-2.js 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v node regexredux.node-2.js 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node regexredux.node-2.js 0 < ../../regexredux-input5000000.txt
