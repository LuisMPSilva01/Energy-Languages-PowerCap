compile:
	/opt/src/ghc9.4.4/bin/ghc --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -XScopedTypeVariables fannkuchredux.ghc-5.hs -v -o fannkuchredux.ghc-5.ghc_run 
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fannkuchredux.ghc-3.ghc_run +RTS -N4 -RTS 12" Haskell fannkuch-redux

run:
	./fannkuchredux.ghc-3.ghc_run +RTS -N4 -RTS 12

mem:
	/usr/bin/time -v  ./fannkuchredux.ghc-3.ghc_run +RTS -N4 -RTS 12

valgrind:
	valgrind --tool=massif --stacks=yes  ./fannkuchredux.ghc-3.ghc_run +RTS -N4 -RTS 12
