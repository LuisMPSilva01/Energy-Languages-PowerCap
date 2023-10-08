compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "perl binarytrees.perl-3.perl 21" Perl binary-trees

run:
	perl binarytrees.perl-3.perl 21

mem:
	/usr/bin/time -v  perl binarytrees.perl-3.perl 21

valgrind:
	valgrind --tool=massif --stacks=yes  perl binarytrees.perl-3.perl 21
