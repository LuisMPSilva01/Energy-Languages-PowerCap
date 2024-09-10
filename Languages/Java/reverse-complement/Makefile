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
	$(JAVAC_PATH) -d .  revcomp.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)   revcomp 0 < ../../../inputs/revcomp-input25000000.txt" Java reverse-complement $(NTIMES)

run:
	$(JAVA_PATH)   revcomp 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v $(JAVA_PATH)   revcomp 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH)   revcomp 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.class