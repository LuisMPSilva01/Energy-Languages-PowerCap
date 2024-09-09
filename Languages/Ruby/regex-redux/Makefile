NTIMES=20

export NTIMES

include ../../../config.env

ifndef RUBY_PATH
$(error RUBY_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RUBY_PATH) -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt" Ruby regex-redux $(NTIMES)

run:
	$(RUBY_PATH) -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v $(RUBY_PATH) -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(RUBY_PATH) -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt
