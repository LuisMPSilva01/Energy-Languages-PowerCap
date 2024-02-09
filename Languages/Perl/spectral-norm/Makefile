NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "perl spectralnorm.perl-4.perl 5500" Perl spectral-norm $(NTIMES)

run:
	perl spectralnorm.perl-4.perl 5500

mem:
	/usr/bin/time -v  perl spectralnorm.perl-4.perl 5500

valgrind:
	valgrind --tool=massif --stacks=yes  perl spectralnorm.perl-4.perl 5500
