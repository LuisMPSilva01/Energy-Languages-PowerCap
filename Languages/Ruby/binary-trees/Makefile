compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 binarytrees.yarv-5.yarv 21" Ruby binary-trees

run:
	ruby -W0 binarytrees.yarv-5.yarv 21

mem:
	/usr/bin/time -v ruby -W0 binarytrees.yarv-5.yarv 21

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 binarytrees.yarv-5.yarv 21
