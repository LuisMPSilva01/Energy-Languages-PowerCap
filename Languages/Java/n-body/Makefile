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
	$(JAVAC_PATH) -d .  nbody.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)   nbody 50000000" Java n-body $(NTIMES)

run:
	$(JAVA_PATH)   nbody 50000000

mem:
	/usr/bin/time -v $(JAVA_PATH)   nbody 50000000

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH)   nbody 50000000

clean:
	rm -f *.class