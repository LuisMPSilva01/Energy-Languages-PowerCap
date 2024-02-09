NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO fasta.python3-3.py 25000000" Python fasta $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO fasta.python3-3.py 25000000

make mem:
	/usr/bin/time -v  /usr/local/bin/python3.11 -OO fasta.python3-3.py 25000000
