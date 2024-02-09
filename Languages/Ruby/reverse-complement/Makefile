NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt" Ruby reverse-complement $(NTIMES)

run:
	ruby -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ruby -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 revcomp.yarv-3.yarv 0 < ../../../inputs/revcomp-input25000000.txt
