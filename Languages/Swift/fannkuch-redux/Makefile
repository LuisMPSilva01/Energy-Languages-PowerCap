NTIMES=1

export NTIMES

compile:
	/opt/swift/usr/bin/swiftc fannkuchredux.swift-3.swift -Ounchecked -whole-module-optimization  -o fannkuchredux.swift-3.swift_run

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
