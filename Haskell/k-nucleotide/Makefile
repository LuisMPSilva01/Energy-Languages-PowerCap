compile:
	/opt/src/ghc9.4.4/bin/ghc --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -funbox-strict-fields -XBinaryLiterals -XDerivingStrategies -XGeneralizedNewtypeDeriving -XOverloadedStrings -XScopedTypeVariables -XTypeApplications -package hashable -package unordered-containers -package pvar -package ghc-compact knucleotide.ghc-3.hs -o knucleotide.ghc-3.ghc_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < knucleotide-input25000000.txt" Haskell fannkuch-redux

run:
	./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v  ./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./knucleotide.ghc-3.ghc_run +RTS -N4 -K2048M -RTS 0 < knucleotide-input25000000.txt
