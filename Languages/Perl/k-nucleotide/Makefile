NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt" Perl k-nucleotide $(NTIMES)

run:
	$(PERL_PATH) knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v $(PERL_PATH) knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(PERL_PATH) knucleotide.perl 0 < ../../../inputs/knucleotide-input25000000.txt
