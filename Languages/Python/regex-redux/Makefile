NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt" Python regex-redux $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt
