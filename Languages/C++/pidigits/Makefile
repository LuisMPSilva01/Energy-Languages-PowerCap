NTIMES=500

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++14 pidigits.gpp-5.c++ -o pidigits.gpp-5.c++.o &&  /usr/bin/g++ pidigits.gpp-5.c++.o -o pidigits.gpp-5.gpp_run -lgmp 
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./pidigits.gpp-5.gpp_run 10000" C++ pidigits $(NTIMES)
run:
	./pidigits.gpp-5.gpp_run 10000

mem:
	/usr/bin/time -v ./pidigits.gpp-5.gpp_run 10000

valgrind:
	valgrind --tool=massif --stacks=yes ./pidigits.gpp-5.gpp_run 10000

clean:
	rm -f *.gpp_run
	rm -f *.o