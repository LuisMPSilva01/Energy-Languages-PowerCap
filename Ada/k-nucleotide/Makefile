compile:
	gnatchop -r -w knucleotide.gnat-2.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f knucleotide.adb -o knucleotide.gnat-2.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp knucleotide.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp data_input.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp string_fragments.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp line_io.adb
	x86_64-linux-gnu-gnatbind-11 -x knucleotide.ali
	x86_64-linux-gnu-gnatlink-11 knucleotide.ali -O3 -fomit-frame-pointer -march=ivybridge -o knucleotide.gnat-2.gnat_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./knucleotide.gnat-2.gnat_run 0 < knucleotide-input25000000.txt" Ada k-nucleotide

run:
	./knucleotide.gnat-2.gnat_run 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.gnat-2.gnat_run 0 < knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.gnat-2.gnat_run 0 < knucleotide-input25000000.txt

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o