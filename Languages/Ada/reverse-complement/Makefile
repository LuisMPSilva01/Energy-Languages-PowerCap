compile:
	gnatchop -r -w revcomp.gnat-2.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f revcomp.adb -o revcomp.gnat-2.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp revcomp.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp line_io.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp text_input.adb
	x86_64-linux-gnu-gnatbind-11 -x revcomp.ali
	x86_64-linux-gnu-gnatlink-11 revcomp.ali -O3 -fomit-frame-pointer -march=ivybridge -o revcomp.gnat-2.gnat_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./revcomp.gnat-2.gnat_run 0 < ../../revcomp-input25000000.txt" Ada reverse-complement

run:
	./revcomp.gnat-2.gnat_run 0 < ../../revcomp-input25000000.txt
	
mem:
	/usr/bin/time -v ./revcomp.gnat-2.gnat_run 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.gnat-2.gnat_run 0 < ../../revcomp-input25000000.txt

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o