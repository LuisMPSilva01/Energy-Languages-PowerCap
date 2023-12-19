NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO binarytrees.py 21" Python binary-trees $(NTIMES)

run:
	/usr/local/bin/python3.11  -OO binarytrees.py 21

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO binarytrees.py 21
