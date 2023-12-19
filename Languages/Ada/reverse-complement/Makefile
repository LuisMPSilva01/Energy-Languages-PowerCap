NTIMES=1

export NTIMES

compile:
	gnatchop -r -w revcomp.gnat-2.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f revcomp.adb -o revcomp.gnat-2.gnat_run 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.gnat-2.gnat_run 0 < ../../../inputs/revcomp-input25000000.txt" Ada reverse-complement $(NTIMES)

run:
	./revcomp.gnat-2.gnat_run 0 < ../../../inputs/revcomp-input25000000.txt
	
mem:
	/usr/bin/time -v ./revcomp.gnat-2.gnat_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.gnat-2.gnat_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o
	rm -f output