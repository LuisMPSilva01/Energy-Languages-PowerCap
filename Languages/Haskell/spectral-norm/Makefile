NTIMES=20

export NTIMES

include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -XMagicHash spectralnorm.ghc-4.hs -o spectralnorm.ghc-4.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.ghc-4.ghc_run +RTS -N4 -RTS 5500" Haskell spectral-norm $(NTIMES)

run:
	./spectralnorm.ghc-4.ghc_run +RTS -N4 -RTS 5500

mem:
	/usr/bin/time -v ./spectralnorm.ghc-4.ghc_run +RTS -N4 -RTS 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.ghc-4.ghc_run +RTS -N4 -RTS 5500
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o