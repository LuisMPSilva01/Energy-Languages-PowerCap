compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 regexredux.gpp-6.c++ -o regexredux.gpp-6.c++.o && /usr/bin/g++ regexredux.gpp-6.c++.o -o regexredux.gpp-6.gpp_run -lpcre2-8 -lpthread 
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.gpp-6.gpp_run 0 < regexredux-input5000000.txt" C++ regex-redux

run:
	./regexredux.gpp-6.gpp_run 0 < regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.gpp-6.gpp_run 0 < regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.gpp-6.gpp_run 0 < regexredux-input5000000.txt

clean:
	rm -f *.gpp_run
	rm -f *.o