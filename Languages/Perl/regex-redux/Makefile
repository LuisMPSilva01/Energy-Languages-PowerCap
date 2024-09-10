NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt" Perl regex-redux $(NTIMES)

run:
	$(PERL_PATH) regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v $(PERL_PATH) regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(PERL_PATH) regexredux.perl-4.perl 0 < ../../../inputs/regexredux-input5000000.txt
