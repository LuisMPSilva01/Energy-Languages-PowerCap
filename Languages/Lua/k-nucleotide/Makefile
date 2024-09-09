NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  knucleotide.lua-2.lua 0 < ../../../inputs/knucleotide-input25000000.txt" Lua k-nucleotide $(NTIMES)

run:
	$(LUA_PATH)  knucleotide.lua-2.lua 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v $(LUA_PATH)  knucleotide.lua-2.lua 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(LUA_PATH)  knucleotide.lua-2.lua 0 < ../../../inputs/knucleotide-input25000000.txt
