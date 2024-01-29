NTIMES=1

export NTIMES

compile:
	/opt/src/ghc9.4.4/bin/ghc --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -funbox-strict-fields -XBinaryLiterals -XDerivingStrategies -XGeneralizedNewtypeDeriving -XOverloadedStrings -XScopedTypeVariables -XTypeApplications -package hashable -package unordered-containers -package pvar -package ghc-compact -package vector knucleotide.ghc-3.hs -o knucleotide.ghc-3.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < ../../../inputs/knucleotide-input25000000.txt" Haskell k-nucleotide $(NTIMES)

run:
	./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < ../../../inputs/knucleotide-input25000000.txt

mem: -package text-builder-linear
	/usr/bin/time -v  ./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < ../../../inputs/knucleotide-input25000000.txt
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o
