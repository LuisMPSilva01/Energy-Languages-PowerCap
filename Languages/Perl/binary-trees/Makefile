NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) binarytrees.perl-3.perl 21" Perl binary-trees $(NTIMES)

run:
	$(PERL_PATH) binarytrees.perl-3.perl 21

mem:
	/usr/bin/time -v  $(PERL_PATH) binarytrees.perl-3.perl 21

valgrind:
	valgrind --tool=massif --stacks=yes  $(PERL_PATH) binarytrees.perl-3.perl 21
