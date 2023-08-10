compile:
	gnatchop -r -w fannkuchredux.gnat-3.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f fannkuchredux.adb -o fannkuchredux.gnat-3.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp fannkuchredux.adb
	x86_64-linux-gnu-gnatbind-11 -x fannkuchredux.ali
	x86_64-linux-gnu-gnatlink-11 fannkuchredux.ali -O3 -fomit-frame-pointer -march=ivybridge -o fannkuchredux.gnat-3.gnat_run


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fannkuchredux.gnat-3.gnat_run 12" Ada fannkuch-redux

run:
	./fannkuchredux.gnat-3.gnat_run 12

mem:
	/usr/bin/time -v  ./fannkuchredux.gnat-3.gnat_run 12

valgrind:
	valgrind --tool=massif --stacks=yes  ./fannkuchredux.gnat-3.gnat_run 12

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o