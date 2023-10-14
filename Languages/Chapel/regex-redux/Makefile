compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast regexredux.chapel-3.chpl -o regexredux.chapel-3.chapel_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.chapel-3.chapel_run --n=0 < regexredux-input5000000.txt" Chapel regex-redux

run:
	./regexredux.chapel-3.chapel_run --n=0 < regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.chapel-3.chapel_run --n=0 < regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.chapel-3.chapel_run --n=0 < regexredux-input5000000.txt
