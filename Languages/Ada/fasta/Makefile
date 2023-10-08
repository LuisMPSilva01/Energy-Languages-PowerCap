compile:
	gnatchop -r -w fasta.gnat-5.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f fasta.adb -o fasta.gnat-5.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp fasta.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp sequence.ads
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp sequence-creation.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp sequence-data.ads
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp lcg_random.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp line_io.adb
	x86_64-linux-gnu-gnatbind-11 -x fasta.ali
	x86_64-linux-gnu-gnatlink-11 fasta.ali -O3 -fomit-frame-pointer -march=ivybridge -o fasta.gnat-5.gnat_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fasta.gnat-5.gnat_run 25000000" Ada fasta

run:
	./fasta.gnat-5.gnat_run 25000000

mem:
	/usr/bin/time -v ./fasta.gnat-5.gnat_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.gnat-5.gnat_run 25000000

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o