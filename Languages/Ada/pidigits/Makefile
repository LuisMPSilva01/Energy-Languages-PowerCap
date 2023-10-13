compile:
	gnatchop -r -w pidigits.gnat-2.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f pidigits.adb -o pidigits.gnat-2.gnat_run 

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