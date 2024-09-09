NTIMES=20

export NTIMES

include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -XScopedTypeVariables -package vector fannkuchredux.ghc-5.hs -v -o fannkuchredux.ghc-5.ghc_run 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.ghc-5.ghc_run +RTS -N4 -RTS 12" Haskell fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.ghc-5.ghc_run +RTS -N4 -RTS 12

mem:
	/usr/bin/time -v  ./fannkuchredux.ghc-5.ghc_run +RTS -N4 -RTS 12

valgrind:
	valgrind --tool=massif --stacks=yes  ./fannkuchredux.ghc-5.ghc_run +RTS -N4 -RTS 12

clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o