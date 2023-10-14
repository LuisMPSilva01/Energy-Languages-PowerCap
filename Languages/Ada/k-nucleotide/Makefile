compile:
	gnatchop -r -w knucleotide.gnat-2.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f knucleotide.adb -o knucleotide.gnat-2.gnat_run


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./knucleotide.gnat-2.gnat_run 0 < ../../inputs/knucleotide-input25000000.txt" Ada k-nucleotide

run:
	./knucleotide.gnat-2.gnat_run 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.gnat-2.gnat_run 0 < ../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.gnat-2.gnat_run 0 < ../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o