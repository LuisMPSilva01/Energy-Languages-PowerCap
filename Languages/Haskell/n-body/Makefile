NTIMES=20

export NTIMES

include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts  nbody.ghc-2.hs -o nbody.ghc-2.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.ghc-2.ghc_run +RTS -N4 -RTS 50000000" Haskell n-body $(NTIMES)

run:
	./nbody.ghc-2.ghc_run +RTS -N4 -RTS 50000000

mem:
	/usr/bin/time -v  ./nbody.ghc-2.ghc_run +RTS -N4 -RTS 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./nbody.ghc-2.ghc_run +RTS -N4 -RTS 50000000
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o