compile:
	cp binarytrees.python3 binarytrees.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/local/bin/python3.11 -OO binarytrees.py 21" Python binary-trees

run:
	/usr/local/bin/python3.11  -OO binarytrees.py 21

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO binarytrees.py 21
