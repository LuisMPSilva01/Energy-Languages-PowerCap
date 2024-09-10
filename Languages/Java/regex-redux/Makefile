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
	$(JAVAC_PATH) -d .  regexredux.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)   regexredux 0 < ../../../inputs/regexredux-input5000000.txt" Java regex-redux $(NTIMES)

run:
	$(JAVA_PATH)   regexredux 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v $(JAVA_PATH)   regexredux 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH)   regexredux 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.class