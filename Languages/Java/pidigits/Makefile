compile:
	/usr/lib/jvm/jdk-20/bin/javac -d . -cp . pidigits.java
xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java -Djava.library.path=Include/java  pidigits 10000" Java pidigits

run:
	/usr/lib/jvm/jdk-20/bin/java -Djava.library.path=Include/java -cp . pidigits 10000

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java -Djava.library.path=Include/java  pidigits 10000

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java -Djava.library.path=Include/java  pidigits 10000

