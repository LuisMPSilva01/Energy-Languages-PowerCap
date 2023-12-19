NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO knucleotide.python3-3.py 0 < ../../../inputs/knucleotide-input25000000.txt" Python k-nucleotide $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO knucleotide.python3-3.py 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO knucleotide.python3-3.py 0 < ../../../inputs/knucleotide-input25000000.txt
