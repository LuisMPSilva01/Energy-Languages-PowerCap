compile:
	cp knucleotide.python3-3.python3 knucleotide.python3-3.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python3.11 -OO knucleotide.python3-3.py 0 < knucleotide-input25000000.txt" Python k-nucleotide

run:
	python3.11 -OO knucleotide.python3-3.py 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v python3.11 -OO knucleotide.python3-3.py 0 < knucleotide-input25000000.txt
