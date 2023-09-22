compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "perl revcomp.perl-3.perl 0 < ../../revcomp-input25000000.txt" Perl reverse-complement

run:
	perl revcomp.perl-3.perl 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v perl revcomp.perl-3.perl 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes perl revcomp.perl-3.perl 0 < ../../revcomp-input25000000.txt
