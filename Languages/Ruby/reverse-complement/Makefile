NTIMES=20

export NTIMES

include ../../../config.env

ifndef RUBY_PATH
$(error RUBY_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RUBY_PATH) -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt" Ruby reverse-complement $(NTIMES)

run:
	$(RUBY_PATH) -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v $(RUBY_PATH) -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(RUBY_PATH) -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt
