compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge  pidigits.gcc-2.c -o pidigits.gcc-2.gcc_run -lgmp
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./pidigits.gcc-2.gcc_run 10000" C pidigits

run:
	./pidigits.gcc-2.gcc_run 10000
mem:
	/usr/bin/time -v ./pidigits.gcc-2.gcc_run 10000

clean:
	rm -f *.gcc_run