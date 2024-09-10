NTIMES=20

export NTIMES

include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -funfolding-use-threshold=32 -XMagicHash -XUnboxedTuples revcomp.ghc-4.hs -o revcomp.ghc-4.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input25000000.txt" Haskell reverse-complement $(NTIMES)

run:
	./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v  ./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input25000000.txt
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o