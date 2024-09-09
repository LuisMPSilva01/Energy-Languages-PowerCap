NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp spectralnorm.gcc-6.c -o spectralnorm.gcc-6.gcc_run 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.gcc-6.gcc_run 5500" C spectral-norm $(NTIMES)

run:
	./spectralnorm.gcc-6.gcc_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.gcc-6.gcc_run 5500

clean:
	rm -f *.gcc_run