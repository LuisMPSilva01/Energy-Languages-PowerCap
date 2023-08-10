compile:
	gnatchop -r -w regexredux.gnat-5.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f regexredux.adb -o regexredux.gnat-5.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp regexredux.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp block_input.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp dna.ads
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp dna-matching.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp dna-replacing.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp preprocessing.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp u.ads
	x86_64-linux-gnu-gnatbind-11 -x regexredux.ali
	x86_64-linux-gnu-gnatlink-11 regexredux.ali -O3 -fomit-frame-pointer -march=ivybridge -o regexredux.gnat-5.gnat_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.gnat-5.gnat_run 0 < ../../regexredux-input5000000.txt" Ada regex-redux

run:
	./regexredux.gnat-5.gnat_run 0 < ../../regexredux-input5000000.txt
	
mem:
	/usr/bin/time -v ./regexredux.gnat-5.gnat_run 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.gnat-5.gnat_run 0 < ../../regexredux-input5000000.txt

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o