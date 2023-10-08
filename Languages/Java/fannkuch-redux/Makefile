compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  fannkuchredux.java
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java   fannkuchredux 12" Java fannkuch-redux

run:
	/usr/lib/jvm/jdk-20/bin/java   fannkuchredux 12

mem:
	/usr/bin/time -v  /usr/lib/jvm/jdk-20/bin/java   fannkuchredux 12

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/lib/jvm/jdk-20/bin/java   fannkuchredux 12

clean:
	rm -f *.class