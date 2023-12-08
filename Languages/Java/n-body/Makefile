NTIMES=1

export NTIMES

compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  nbody.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java   nbody 50000000" Java n-body $(NTIMES)

run:
	/usr/lib/jvm/jdk-20/bin/java   nbody 50000000

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java   nbody 50000000

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   nbody 50000000

clean:
	rm -f *.class