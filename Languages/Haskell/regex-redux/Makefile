NTIMES=20

export NTIMES

include ../../../config.env

ifndef GHC_PATH
$(error GHC_PATH is not set.)
endif

compile:
	$(GHC_PATH) --make -fllvm -O2 -XBangPatterns -threaded -rtsopts -XForeignFunctionInterface -XCApiFFI -lpcre2-8 -optc "-DPCRE2_CODE_UNIT_WIDTH=8" -package vector regexredux.ghc-3.hs -o regexredux.ghc-3.ghc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.ghc-3.ghc_run +RTS -N4 -H250M -RTS 0 < ../../../inputs/regexredux-input5000000.txt" Haskell regex-redux $(NTIMES)

run:
	./regexredux.ghc-3.ghc_run +RTS -N4 -H250M -RTS 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  ./regexredux.ghc-3.ghc_run +RTS -N4 -H250M -RTS 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./regexredux.ghc-3.ghc_run +RTS -N4 -H250M -RTS 0 < ../../../inputs/regexredux-input5000000.txt
clean:
	rm -f *.ghc_run
	rm -f *.hi
	rm -f *.o
