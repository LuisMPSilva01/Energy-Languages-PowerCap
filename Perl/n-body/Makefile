compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "perl nbody.perl-2.perl 50000000" Perl n-body

run:
	perl nbody.perl-2.perl 50000000

mem:
	/usr/bin/time -v perl nbody.perl-2.perl 50000000

valgrind:
	valgrind --tool=massif --stacks=yes perl nbody.perl-2.perl 50000000
