NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 nbody.yarv-2.yarv 50000000" Ruby n-body $(NTIMES)

run:
	ruby -W0 nbody.yarv-2.yarv 50000000

mem:
	/usr/bin/time -v ruby -W0 nbody.yarv-2.yarv 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 nbody.yarv-2.yarv 50000000
