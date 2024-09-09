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
	$(RACO_PATH) make revcomp.racket-2.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RACKET_PATH) ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt" Racket reverse-complement $(NTIMES)

run:
	$(RACKET_PATH) ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v $(RACKET_PATH) ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(RACKET_PATH) ./compiled/revcomp.racket-2_racket.zo 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -rf compiled/
