NTIMES=20

export NTIMES

include ../../../config.env

ifndef SWIFTC_PATH
$(error SWIFTC_PATH is not set.)
endif

compile:
	$(SWIFTC_PATH) mandelbrot.swift-3.swift -Ounchecked -whole-module-optimization  -o mandelbrot.swift-3.swift_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.swift-3.swift_run 16000" Swift mandelbrot $(NTIMES)

run:
	./mandelbrot.swift-3.swift_run 16000

mem:
	/usr/bin/time -v  ./mandelbrot.swift-3.swift_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes  ./mandelbrot.swift-3.swift_run 16000

clean:
	rm *.swift_run
