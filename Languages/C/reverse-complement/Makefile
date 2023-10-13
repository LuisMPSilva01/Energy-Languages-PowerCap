NTIMES=5

export NTIMES

compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -pthread revcomp.gcc-7.c -o revcomp.gcc-7.gcc_run 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.gcc-7.gcc_run 0 < revcomp-input25000000.txt" C reverse-complement $(NTIMES)

run:
	./revcomp.gcc-7.gcc_run 0 < revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.gcc-7.gcc_run 0 < revcomp-input25000000.txt

clean:
	rm -f *.gcc_run