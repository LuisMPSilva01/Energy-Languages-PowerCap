NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "perl fasta.perl 25000000" Perl fasta $(NTIMES)

run:
	perl fasta.perl 25000000
mem:
	/usr/bin/time -v perl fasta.perl 25000000

valgrind:
	valgrind --tool=massif --stacks=yes perl fasta.perl 25000000

