NTIMES=20

export NTIMES

include ../../../config.env

ifndef RACO_PATH
$(error RACO_PATH is not set.)
endif

ifndef RACKET_PATH
$(error RACKET_PATH is not set.)
endif

compile:
	$(RACO_PATH) make regexredux.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RACKET_PATH) ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt" Racket regex-redux $(NTIMES)

run:
	$(RACKET_PATH) ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v $(RACKET_PATH) ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(RACKET_PATH) ./compiled/regexredux_racket.zo 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -rf compiled/
