NTIMES=1

export NTIMES

compile:
	/usr/lib/jvm/jdk-20/bin/javac -d .  mandelbrot.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/lib/jvm/jdk-20/bin/java   mandelbrot 16000" Java mandelbrot $(NTIMES)

run:
	/usr/lib/jvm/jdk-20/bin/java   mandelbrot 16000

mem:
	/usr/bin/time -v /usr/lib/jvm/jdk-20/bin/java   mandelbrot 16000

valgrind:
	valgrind --tool=massif --stacks=yes /usr/lib/jvm/jdk-20/bin/java   mandelbrot 16000

clean:
	rm -f *.class