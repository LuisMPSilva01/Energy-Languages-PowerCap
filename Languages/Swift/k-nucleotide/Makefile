NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) knucleotide.swift-2.swift -Ounchecked -whole-module-optimization  -o knucleotide.swift-2.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.swift-2.swift_run 0 < ../../../inputs/knucleotide-input25000000.txt" Swift k-nucleotide $(NTIMES)

run:
	./knucleotide.swift-2.swift_run 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.swift-2.swift_run 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.swift-2.swift_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm *.swift_run
