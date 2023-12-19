NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	export LUA_CPATH="/usr/lib/x86_64-linux-gnu/lua/5.4/?.so;;" && \
	sudo ../../../RAPL/main "lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt" Lua regex-redux $(NTIMES)

run:
	export LUA_CPATH="/usr/lib/x86_64-linux-gnu/lua/5.4/?.so;;" && \
	lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt
