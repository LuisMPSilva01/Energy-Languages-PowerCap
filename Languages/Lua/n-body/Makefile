NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  nbody.lua-2.lua 50000000" Lua n-body $(NTIMES)

run:
	$(LUA_PATH)  nbody.lua-2.lua 50000000

mem:
	/usr/bin/time -v  $(LUA_PATH)  nbody.lua-2.lua 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  $(LUA_PATH)  nbody.lua-2.lua 50000000
