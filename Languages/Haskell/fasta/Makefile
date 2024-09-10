NTIMES=20

export NTIMES

include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -XStrict fasta.ghc-6.hs -o fasta.ghc-6.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.ghc-6.ghc_run +RTS -N4 -RTS 25000000" Haskell fasta $(NTIMES)

run:
	./fasta.ghc-6.ghc_run +RTS -N4 -RTS 25000000

mem:
	/usr/bin/time -v  ./fasta.ghc-6.ghc_run +RTS -N4 -RTS 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./fasta.ghc-6.ghc_run +RTS -N4 -RTS 25000000
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o