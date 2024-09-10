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
	$(RACO_PATH) make fasta.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RACKET_PATH) ./compiled/fasta.racket-3_racket.zo 25000000" Racket fasta $(NTIMES)

run:
	$(RACKET_PATH) ./compiled/fasta.racket-3_racket.zo 25000000

mem:
	/usr/bin/time -v $(RACKET_PATH) ./compiled/fasta.racket-3_racket.zo 25000000

valgrind:
	valgrind --tool=massif --stacks=yes $(RACKET_PATH) ./compiled/fasta.racket-3_racket.zo 25000000

clean:
	rm -rf compiled/
