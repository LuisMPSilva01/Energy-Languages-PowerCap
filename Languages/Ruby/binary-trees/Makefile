NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 binarytrees.yarv-5.yarv 21" Ruby binary-trees $(NTIMES)

run:
	ruby -W0 binarytrees.yarv-5.yarv 21

mem:
	/usr/bin/time -v ruby -W0 binarytrees.yarv-5.yarv 21

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 binarytrees.yarv-5.yarv 21
