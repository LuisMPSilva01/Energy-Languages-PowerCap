NTIMES=20

export NTIMES

include ../../../config.env

ifndef JAVAC_PATH
$(error JAVAC_PATH is not set.)
endif

ifndef JAVA_PATH
$(error JAVA_PATH is not set.)
endif

compile:
	$(JAVAC_PATH) -d .  spectralnorm.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)    spectralnorm 5500" Java spectral-norm $(NTIMES)

run:
	$(JAVA_PATH)   spectralnorm 5500

mem:
	/usr/bin/time -v $(JAVA_PATH)   spectralnorm 5500

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH)   spectralnorm 5500

clean:
	rm -f *.class