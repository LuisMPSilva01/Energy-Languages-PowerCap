NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 spectralnorm.yarv-5.yarv 5500" Ruby spectral-norm $(NTIMES)

run:
	ruby -W0 spectralnorm.yarv-5.yarv 5500

mem:
	/usr/bin/time -v ruby -W0 spectralnorm.yarv-5.yarv 5500

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 spectralnorm.yarv-5.yarv 5500

