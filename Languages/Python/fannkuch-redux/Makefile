NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO fannkuchredux.python3-4.py 12" Python fannkuch-redux $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO fannkuchredux.python3-4.py 12
mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO fannkuchredux.python3-4.py 12
