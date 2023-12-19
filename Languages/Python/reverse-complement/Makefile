NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO revcomp.python3-6.py 0 < ../../../inputs/revcomp-input25000000.txt" Python reverse-complement $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO revcomp.python3-6.py 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO revcomp.python3-6.py 0 < ../../../inputs/revcomp-input25000000.txt

