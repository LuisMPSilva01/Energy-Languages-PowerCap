NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  fasta.lua-3.lua 25000000" Lua fasta $(NTIMES)

run:
	$(LUA_PATH)  fasta.lua-3.lua 25000000

mem:
	/usr/bin/time -v  $(LUA_PATH)  fasta.lua-3.lua 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  $(LUA_PATH)  fasta.lua-3.lua 25000000
