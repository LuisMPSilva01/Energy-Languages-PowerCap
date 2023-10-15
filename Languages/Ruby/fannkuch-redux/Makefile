compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 fannkuchredux.yarv-2.yarv 12" Ruby fannkuch-redux

run:
	ruby -W0 fannkuchredux.yarv-2.yarv 12

mem:
	/usr/bin/time -v ruby -W0 fannkuchredux.yarv-2.yarv 12

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 fannkuchredux.yarv-2.yarv 12
