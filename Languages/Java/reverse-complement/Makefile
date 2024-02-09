NTIMES=20

export NTIMES

compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  revcomp.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java   revcomp 0 < ../../../inputs/revcomp-input25000000.txt" Java reverse-complement $(NTIMES)

run:
	/usr/lib/jvm/jdk-20/bin/java   revcomp 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java   revcomp 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   revcomp 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.class