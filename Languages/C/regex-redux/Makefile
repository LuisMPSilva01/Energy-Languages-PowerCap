NTIMES=20

export NTIMES

include ../../../config.env

ifndef GCC_PATH
$(error GCC_PATH is not set.)
endif

compile:
	$(GCC_PATH) -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp regexredux.gcc-5.c -o regexredux.gcc-5.gcc_run -lpcre2-8
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.gcc-5.gcc_run 0 < ../../../inputs/regexredux-input5000000.txt" C regex-redux $(NTIMES)

run:
	./regexredux.gcc-5.gcc_run 0 < ../../../inputs/regexredux-input5000000.txt
mem:
	/usr/bin/time -v ./regexredux.gcc-5.gcc_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.gcc_run