include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -XUnboxedTuples -XUnliftedFFITypes pidigits.ghc-6.hs -o pidigits.ghc-6.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./pidigits.ghc-6.ghc_run +RTS -N4 -RTS 10000" Haskell fannkuch-redux $(NTIMES)

run:
	./pidigits.ghc-6.ghc_run +RTS -N4 -RTS 10000

mem:
	/usr/bin/time -v  ./pidigits.ghc-6.ghc_run +RTS -N4 -RTS 10000

valgrind:
	valgrind --tool=massif --stacks=yes  ./pidigits.ghc-6.ghc_run +RTS -N4 -RTS 10000
