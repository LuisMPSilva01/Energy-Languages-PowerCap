NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  binarytrees.lua-2.lua 21" Lua binary-trees $(NTIMES)

run:
	$(LUA_PATH)  binarytrees.lua-2.lua 21

mem:
	/usr/bin/time -v $(LUA_PATH)  binarytrees.lua-2.lua 21

valgrind:
	valgrind --tool=massif --stacks=yes $(LUA_PATH)  binarytrees.lua-2.lua 21
