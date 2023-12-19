NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "perl regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt" Perl regex-redux $(NTIMES)

run:
	perl regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v perl regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes perl regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt
