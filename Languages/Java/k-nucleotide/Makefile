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
	$(JAVAC_PATH) -d . -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)  -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt" Java k-nucleotide $(NTIMES)

run:
	$(JAVA_PATH)  -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v $(JAVA_PATH)  -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH)   -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.class