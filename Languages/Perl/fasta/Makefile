NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) fasta.perl 25000000" Perl fasta $(NTIMES)

run:
	$(PERL_PATH) fasta.perl 25000000
mem:
	/usr/bin/time -v $(PERL_PATH) fasta.perl 25000000

valgrind:
	valgrind --tool=massif --stacks=yes $(PERL_PATH) fasta.perl 25000000

