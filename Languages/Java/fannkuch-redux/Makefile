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
	$(JAVAC_PATH) -d .  fannkuchredux.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)   fannkuchredux 12" Java fannkuch-redux $(NTIMES)

run:
	$(JAVA_PATH)   fannkuchredux 12

mem:
	/usr/bin/time -v  $(JAVA_PATH)   fannkuchredux 12

valgrind:
	valgrind --tool=massif --stacks=yes  $(JAVA_PATH)   fannkuchredux 12

clean:
	rm -f *.class