compile:
	/opt/src/chapel-1.29.0/bin/chpl --fast fannkuchredux.chapel-2.chpl -o fannkuchredux.chapel-2.chapel_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fannkuchredux.chapel-2.chapel_run --n=12" Chapel fannkuch-redux

run:
	./fannkuchredux.chapel-2.chapel_run --n=12

mem:
	/usr/bin/time -v ./fannkuchredux.chapel-2.chapel_run --n=12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.chapel-2.chapel_run --n=12
