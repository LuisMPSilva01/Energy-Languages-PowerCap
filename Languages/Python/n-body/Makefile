compile:
	cp nbody.python3 nbody.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/local/bin/python3.11 -OO nbody.py 50000000" Python n-body

run:
	/usr/local/bin/python3.11 -OO nbody.py 50000000

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO nbody.py 50000000
