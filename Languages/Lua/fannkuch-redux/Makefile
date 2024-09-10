NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  fannkuchredux.lua 12" Lua fannkuch-redux $(NTIMES)

run:
	$(LUA_PATH)  fannkuchredux.lua 12

mem:
	/usr/bin/time -v $(LUA_PATH)  fannkuchredux.lua 12

valgrind:
	valgrind --tool=massif --stacks=yes $(LUA_PATH)  fannkuchredux.lua 12
