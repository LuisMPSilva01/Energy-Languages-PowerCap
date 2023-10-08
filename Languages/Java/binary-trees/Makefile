compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  binarytrees.java
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java binarytrees 21" Java binary-trees

run:
	/usr/lib/jvm/jdk-20/bin/java binarytrees 21

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java binarytrees 21

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java binarytrees 21

clean:
	rm -f *.class