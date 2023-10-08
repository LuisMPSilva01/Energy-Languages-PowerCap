compile:
	cp nbody.python3 nbody.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python3.11 -OO nbody.py 50000000" Python n-body

run:
	python3.11 -OO nbody.py 50000000

mem:
	/usr/bin/time -v python3.11 -OO nbody.py 50000000
