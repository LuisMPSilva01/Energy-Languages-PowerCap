NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) fasta.swift-3.swift -Ounchecked -whole-module-optimization  -o fasta.swift-3.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.swift-3.swift_run 25000000" Swift fasta $(NTIMES)

run:
	./fasta.swift-3.swift_run 25000000

mem:
	/usr/bin/time -v  ./fasta.swift-3.swift_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./fasta.swift-3.swift_run 25000000

clean:
	rm *.swift_run
