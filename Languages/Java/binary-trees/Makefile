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
	$(JAVAC_PATH) -d .  binarytrees.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH) binarytrees 21" Java binary-trees $(NTIMES)

run:
	$(JAVA_PATH) binarytrees 21

mem:
	/usr/bin/time -v $(JAVA_PATH) binarytrees 21

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH) binarytrees 21

clean:
	rm -f *.class