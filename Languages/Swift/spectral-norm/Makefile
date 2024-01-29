NTIMES=1

export NTIMES

compile:
	/opt/swift/usr/bin/swiftc spectralnorm.swift-3.swift -Ounchecked -whole-module-optimization  -o spectralnorm.swift-3.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.swift-3.swift_run 5500" Swift spectral-norm $(NTIMES)

run:
	./spectralnorm.swift-3.swift_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.swift-3.swift_run 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.swift-3.swift_run 5500

clean:
	rm *.swift_run
