NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) binarytrees.swift-9.swift -Ounchecked -whole-module-optimization -I Include/swift/apr -o binarytrees.swift-9.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.swift-9.swift_run 21" Swift binary-trees $(NTIMES)

run:
	./binarytrees.swift-9.swift_run 21

mem:
	/usr/bin/time -v ./binarytrees.swift-9.swift_run 21

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.swift-9.swift_run 21

clean:
	rm *.swift_run
