compile:
	gnatchop -r -w binarytrees.gnat-4.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f binarytrees.adb -o binarytrees.gnat-4.gnat_run -largs -lapr-1
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp binarytrees.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp trees.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp apache_runtime.ads
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp apache_runtime-pools.adb
	x86_64-linux-gnu-gnatbind-11 -x binarytrees.ali
	x86_64-linux-gnu-gnatlink-11 binarytrees.ali -O3 -fomit-frame-pointer -march=ivybridge -o binarytrees.gnat-4.gnat_run -lapr-1

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./binarytrees.gnat-4.gnat_run 21" Ada binary-trees

run:
	./binarytrees.gnat-4.gnat_run 21

mem:
	/usr/bin/time -v ./binarytrees.gnat-4.gnat_run 21

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.gnat-4.gnat_run 21

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o