NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt" Ruby regex-redux $(NTIMES)

run:
	ruby -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ruby -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 regexredux.yarv-2.yarv 0 < ../../../inputs/regexredux-input5000000.txt
