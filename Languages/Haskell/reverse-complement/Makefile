compile:
	/opt/src/ghc9.4.4/bin/ghc --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -funfolding-use-threshold=32 -XMagicHash -XUnboxedTuples revcomp.ghc-4.hs -o revcomp.ghc-4.ghc_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input100000001.txt" Haskell reverse-complement

run:
	./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input100000001.txt

mem:
	/usr/bin/time -v  ./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input100000001.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./revcomp.ghc-4.ghc_run +RTS -N4 -RTS 0 < ../../../inputs/revcomp-input100000001.txt
