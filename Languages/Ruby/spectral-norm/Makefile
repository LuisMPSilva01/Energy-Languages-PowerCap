NTIMES=20

export NTIMES

include ../../../config.env

ifndef RUBY_PATH
$(error RUBY_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RUBY_PATH) -W0 spectralnorm.yarv-5.yarv 5500" Ruby spectral-norm $(NTIMES)

run:
	$(RUBY_PATH) -W0 spectralnorm.yarv-5.yarv 5500

mem:
	/usr/bin/time -v $(RUBY_PATH) -W0 spectralnorm.yarv-5.yarv 5500

valgrind:
	valgrind --tool=massif --stacks=yes $(RUBY_PATH) -W0 spectralnorm.yarv-5.yarv 5500

