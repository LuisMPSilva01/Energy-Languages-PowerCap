NTIMES=1

export NTIMES

compile:
	/opt/src/ghc9.4.4/bin/ghc --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -fno-cse -package ghc-compact binarytrees.ghc-8.hs -o binarytrees.ghc-8.ghc_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.ghc-8.ghc_run +RTS -N4 -K128M -H -RTS 21" Haskell binary-trees $(NTIMES)

run:
	./binarytrees.ghc-8.ghc_run +RTS -N4 -K128M -H -RTS 21

clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o
mem:
	/usr/bin/time -v  ./binarytrees.ghc-8.ghc_run +RTS -N4 -K128M -H -RTS 21

valgrind:
	valgrind --tool=massif --stacks=yes  ./binarytrees.ghc-8.ghc_run +RTS -N4 -K128M -H -RTS 21
