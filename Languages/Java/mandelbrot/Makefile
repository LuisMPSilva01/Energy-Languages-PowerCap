NTIMES=20

export NTIMES

include ../../../config.env

ifndef JAVAC_PATH
$(error JAVAC_PATH is not set.)
endif

ifndef JAVA_PATH
$(error JAVA_PATH is not set.)
endif

compile:
	$(JAVAC_PATH) -d .  mandelbrot.java
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(JAVA_PATH)   mandelbrot 16000" Java mandelbrot $(NTIMES)

run:
	$(JAVA_PATH)   mandelbrot 16000

mem:
	/usr/bin/time -v $(JAVA_PATH)   mandelbrot 16000

valgrind:
	valgrind --tool=massif --stacks=yes $(JAVA_PATH)   mandelbrot 16000

clean:
	rm -f *.class