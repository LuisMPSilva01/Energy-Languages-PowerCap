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
	$(RACO_PATH) make spectralnorm.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(RACKET_PATH) ./compiled/spectralnorm.racket-3_racket.zo 5500" Racket spectral-norm $(NTIMES)

run:
	$(RACKET_PATH) ./compiled/spectralnorm.racket-3_racket.zo 5500

mem:
	/usr/bin/time -v $(RACKET_PATH) ./compiled/spectralnorm.racket-3_racket.zo 5500

valgrind:
	valgrind --tool=massif --stacks=yes $(RACKET_PATH) ./compiled/spectralnorm.racket-3_racket.zo 5500

clean:
	rm -rf compiled/
