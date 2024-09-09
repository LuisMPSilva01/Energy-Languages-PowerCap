NTIMES=20

export NTIMES

include ../../../config.env

ifndef PYTHON_PATH
$(error PYTHON_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PYTHON_PATH) -OO fasta.python3-3.py 25000000" Python fasta $(NTIMES)

run:
	$(PYTHON_PATH) -OO fasta.python3-3.py 25000000

make mem:
	/usr/bin/time -v  $(PYTHON_PATH) -OO fasta.python3-3.py 25000000
