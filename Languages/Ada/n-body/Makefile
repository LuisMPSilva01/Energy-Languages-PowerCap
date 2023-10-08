compile:
	gnatchop -r -w nbody.gnat-2.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f nbody.adb -o nbody.gnat-2.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp nbody.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp nbody_pck.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp root.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp systems.adb
	x86_64-linux-gnu-gnatbind-11 -x nbody.ali
	x86_64-linux-gnu-gnatlink-11 nbody.ali -O3 -fomit-frame-pointer -march=ivybridge -o nbody.gnat-2.gnat_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./nbody.gnat-2.gnat_run 50000000" Ada n-body

run:
	./nbody.gnat-2.gnat_run 50000000
	
mem:
	/usr/bin/time -v ./nbody.gnat-2.gnat_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.gnat-2.gnat_run 50000000


clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o