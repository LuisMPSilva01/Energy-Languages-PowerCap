compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "perl fannkuchredux.perl-2.perl 12" Perl fannkuch-redux

run:
	perl fannkuchredux.perl-2.perl 12

mem:
	/usr/bin/time -v perl fannkuchredux.perl-2.perl 12

valgrind:
	valgrind --tool=massif --stacks=yes perl fannkuchredux.perl-2.perl 12
