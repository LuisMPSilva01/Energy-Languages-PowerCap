NTIMES=20

export NTIMES

compile:
	/usr/lib/jvm/jdk-20/bin/javac -d . -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java  -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt" Java k-nucleotide $(NTIMES)

run:
	/usr/lib/jvm/jdk-20/bin/java  -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java  -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   -cp .:/opt/src/java-libs/fastutil-8.3.1.jar knucleotide 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.class