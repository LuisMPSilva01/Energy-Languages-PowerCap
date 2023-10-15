compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 nbody.yarv-2.yarv 50000000" Ruby n-body

run:
	ruby -W0 nbody.yarv-2.yarv 50000000

mem:
	/usr/bin/time -v ruby -W0 nbody.yarv-2.yarv 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 nbody.yarv-2.yarv 50000000
