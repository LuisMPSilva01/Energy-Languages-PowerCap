NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) fannkuchredux.swift-3.swift -Ounchecked -whole-module-optimization  -o fannkuchredux.swift-3.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.swift-3.swift_run 12" Swift fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.swift-3.swift_run 12

mem:
	/usr/bin/time -v ./fannkuchredux.swift-3.swift_run 12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.swift-3.swift_run 12

clean:
	rm *.swift_run
