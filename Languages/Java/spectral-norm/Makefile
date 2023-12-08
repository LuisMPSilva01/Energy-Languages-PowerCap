NTIMES=1

export NTIMES

compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  spectralnorm.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java    spectralnorm 5500" Java spectral-norm $(NTIMES)

run:
	/usr/lib/jvm/jdk-20/bin/java   spectralnorm 5500

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java   spectralnorm 5500

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   spectralnorm 5500

clean:
	rm -f *.class