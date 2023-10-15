compile:
	cp regexredux.python3 regexredux.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/local/bin/python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt" Python regex-redux

run:
	/usr/local/bin/python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt
