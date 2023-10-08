compile:
	gnatchop -r -w pidigits.gnat-2.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f pidigits.adb -o pidigits.gnat-2.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp pidigits.adb
	x86_64-linux-gnu-gnatbind-11 -x pidigits.ali
	x86_64-linux-gnu-gnatlink-11 pidigits.ali -O3 -fomit-frame-pointer -march=ivybridge -o pidigits.gnat-2.gnat_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./pidigits.gnat-2.gnat_run 10000" Ada pidigits

run:
	./pidigits.gnat-2.gnat_run 10000
	
mem:
	/usr/bin/time -v  ./pidigits.gnat-2.gnat_run 10000

valgrind:
	valgrind --tool=massif --stacks=yes  ./pidigits.gnat-2.gnat_run 10000

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o