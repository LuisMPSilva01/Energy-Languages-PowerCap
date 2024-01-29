NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 fannkuchredux.yarv-2.yarv 12" Ruby fannkuch-redux $(NTIMES)

run:
	ruby -W0 fannkuchredux.yarv-2.yarv 12

mem:
	/usr/bin/time -v ruby -W0 fannkuchredux.yarv-2.yarv 12

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 fannkuchredux.yarv-2.yarv 12
