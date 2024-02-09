NTIMES=20

export NTIMES

compile:
	/opt/swift/usr/bin/swiftc nbody.swift-5.swift -Ounchecked -whole-module-optimization  -o nbody.swift-5.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.swift-5.swift_run 50000000" Swift n-body $(NTIMES)

run:
	./nbody.swift-5.swift_run 50000000

mem:
	/usr/bin/time -v  ./nbody.swift-5.swift_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./nbody.swift-5.swift_run 50000000

clean:
	rm *.swift_run
