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
	$(RACO_PATH) make nbody.racket-2.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RACKET_PATH) ./compiled/nbody.racket-2_racket.zo 50000000" Racket n-body $(NTIMES)

run:
	$(RACKET_PATH) ./compiled/nbody.racket-2_racket.zo 50000000

mem:
	/usr/bin/time -v  $(RACKET_PATH) ./compiled/nbody.racket-2_racket.zo 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  $(RACKET_PATH) ./compiled/nbody.racket-2_racket.zo 50000000

clean:
	rm -rf compiled/
