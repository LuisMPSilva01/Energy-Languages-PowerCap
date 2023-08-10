compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp spectralnorm.gcc-6.c -o spectralnorm.gcc-6.gcc_run 
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./spectralnorm.gcc-6.gcc_run 5500" C spectral-norm

run:
	./spectralnorm.gcc-6.gcc_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.gcc-6.gcc_run 5500

clean:
	rm -f *.gcc_run