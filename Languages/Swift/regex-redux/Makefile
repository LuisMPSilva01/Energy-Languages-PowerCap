NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) regexredux.swift-2.swift -Ounchecked -whole-module-optimization  -o regexredux.swift-2.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.swift-2.swift_run 0 < ../../../inputs/regexredux-input5000000.txt" Swift regex-redux $(NTIMES)

run:
	./regexredux.swift-2.swift_run 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.swift-2.swift_run 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.swift-2.swift_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm *.swift_run
