NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) fannkuchredux.perl-2.perl 12" Perl fannkuch-redux $(NTIMES)

run:
	$(PERL_PATH) fannkuchredux.perl-2.perl 12

mem:
	/usr/bin/time -v $(PERL_PATH) fannkuchredux.perl-2.perl 12

valgrind:
	valgrind --tool=massif --stacks=yes $(PERL_PATH) fannkuchredux.perl-2.perl 12
