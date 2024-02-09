NTIMES=20

export NTIMES

compile:
	gnatchop -r -w binarytrees.gnat-4.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f binarytrees.adb -o binarytrees.gnat-4.gnat_run -largs -lapr-1



measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.gnat-4.gnat_run 21" Ada binary-trees $(NTIMES)

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
	rm -f output