compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=native -fopenmp regexredux.gcc-5.c -o regexredux.gcc-5.gcc_run -lpcre
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.gcc-5.gcc_run 0 < ../../regexredux-input5000000.txt" C regex-redux

run:
	./regexredux.gcc-5.gcc_run 0 < ../../regexredux-input5000000.txt
mem:
	/usr/bin/time -v ./regexredux.gcc-5.gcc_run 0 < ../../regexredux-input5000000.txt

clean:
	rm -f *.gcc_run