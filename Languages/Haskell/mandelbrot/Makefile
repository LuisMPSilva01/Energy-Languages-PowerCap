NTIMES=1

export NTIMES

compile:
	/opt/src/ghc9.4.4/bin/ghc --make -fllvm -O2 -XBangPatterns -threaded -rtsopts  mandelbrot.ghc-2.hs -o mandelbrot.ghc-2.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.ghc-2.ghc_run +RTS -N4 -RTS 16000" Haskell mandelbrot $(NTIMES)

run:
	./mandelbrot.ghc-2.ghc_run +RTS -N4 -RTS 16000

mem:
	/usr/bin/time -v ./mandelbrot.ghc-2.ghc_run +RTS -N4 -RTS 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.ghc-2.ghc_run +RTS -N4 -RTS 16000
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o