compile:
	gnatchop -r -w regexredux.gnat-5.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f regexredux.adb -o regexredux.gnat-5.gnat_run 

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.gnat-5.gnat_run 0 < ../../../inputs/regexredux-input5000000.txt" Ada regex-redux

run:
	./regexredux.gnat-5.gnat_run 0 < ../../../inputs/regexredux-input5000000.txt
	
mem:
	/usr/bin/time -v ./regexredux.gnat-5.gnat_run 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.gnat-5.gnat_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o