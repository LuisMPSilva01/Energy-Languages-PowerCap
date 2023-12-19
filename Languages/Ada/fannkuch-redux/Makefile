NTIMES=1

export NTIMES

compile:
	gnatchop -r -w fannkuchredux.gnat-3.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f fannkuchredux.adb -o fannkuchredux.gnat-3.gnat_run


measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.gnat-3.gnat_run 12" Ada fannkuch-redux $(NTIMES)

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
	rm -f output