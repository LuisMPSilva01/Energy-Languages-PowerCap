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
	$(RACO_PATH) make fannkuchredux.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RACKET_PATH) ./compiled/fannkuchredux.racket-3_racket.zo 12" Racket fannkuch-redux $(NTIMES)

run:
	$(RACKET_PATH) ./compiled/fannkuchredux.racket-3_racket.zo 12

mem:
	/usr/bin/time -v $(RACKET_PATH) ./compiled/fannkuchredux.racket-3_racket.zo 12

valgrind:
	valgrind --tool=massif --stacks=yes $(RACKET_PATH) ./compiled/fannkuchredux.racket-3_racket.zo 12

clean:
	rm -rf compiled/
