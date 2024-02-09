NTIMES=20

export NTIMES

compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  fasta.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java   fasta 25000000" Java fasta $(NTIMES)

run:
	/usr/lib/jvm/jdk-20/bin/java   fasta 25000000

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java   fasta 25000000

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   fasta 25000000

clean:
	rm -f *.class