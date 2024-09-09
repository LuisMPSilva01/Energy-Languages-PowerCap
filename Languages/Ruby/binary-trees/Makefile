NTIMES=20

export NTIMES

include ../../../config.env

ifndef RUBY_PATH
$(error RUBY_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RUBY_PATH) -W0 binarytrees.yarv-5.yarv 21" Ruby binary-trees $(NTIMES)

run:
	$(RUBY_PATH) -W0 binarytrees.yarv-5.yarv 21

mem:
	/usr/bin/time -v $(RUBY_PATH) -W0 binarytrees.yarv-5.yarv 21

valgrind:
	valgrind --tool=massif --stacks=yes $(RUBY_PATH) -W0 binarytrees.yarv-5.yarv 21
