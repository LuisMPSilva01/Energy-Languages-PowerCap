NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) revcomp.swift -Ounchecked -whole-module-optimization  -o revcomp.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.swift_run 0 < ../../../inputs/revcomp-input25000000.txt" Swift reverse-complement $(NTIMES)

run:
	./revcomp.swift_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v  ./revcomp.swift_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./revcomp.swift_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm *.swift_run
