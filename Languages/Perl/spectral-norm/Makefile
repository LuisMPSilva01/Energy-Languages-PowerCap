NTIMES=20

export NTIMES

include ../../../config.env

ifndef PERL_PATH
$(error PERL_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PERL_PATH) spectralnorm.perl-4.perl 5500" Perl spectral-norm $(NTIMES)

run:
	$(PERL_PATH) spectralnorm.perl-4.perl 5500

mem:
	/usr/bin/time -v  $(PERL_PATH) spectralnorm.perl-4.perl 5500

valgrind:
	valgrind --tool=massif --stacks=yes  $(PERL_PATH) spectralnorm.perl-4.perl 5500
