NTIMES=1

export NTIMES

compile:
	gnatchop -r -w nbody.gnat-2.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f nbody.adb -o nbody.gnat-2.gnat_run 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.gnat-2.gnat_run 50000000" Ada n-body $(NTIMES)

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
	rm -f output