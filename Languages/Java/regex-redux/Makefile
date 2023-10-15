compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  regexredux.java
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java   regexredux 0 < ../../../inputs/regexredux-input5000000.txt" Java regex-redux

run:
	/usr/lib/jvm/jdk-20/bin/java   regexredux 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java   regexredux 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   regexredux 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.class