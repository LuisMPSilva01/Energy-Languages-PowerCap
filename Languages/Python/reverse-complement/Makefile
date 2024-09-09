NTIMES=20

export NTIMES

include ../../../config.env

ifndef PYTHON_PATH
$(error PYTHON_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PYTHON_PATH) -OO revcomp.python3-6.py 0 < ../../../inputs/revcomp-input25000000.txt" Python reverse-complement $(NTIMES)

run:
	$(PYTHON_PATH) -OO revcomp.python3-6.py 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v $(PYTHON_PATH) -OO revcomp.python3-6.py 0 < ../../../inputs/revcomp-input25000000.txt

