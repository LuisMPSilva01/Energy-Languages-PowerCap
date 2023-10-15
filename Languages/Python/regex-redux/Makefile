compile:
	cp regexredux.python3 regexredux.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt" Python regex-redux

run:
	python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v python3.11 -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt
