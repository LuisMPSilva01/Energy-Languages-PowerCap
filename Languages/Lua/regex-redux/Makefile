NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt" Lua regex-redux $(NTIMES)

run:
	$(LUA_PATH) regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt


mem:
	/usr/bin/time -v $(LUA_PATH)  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(LUA_PATH)  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt
