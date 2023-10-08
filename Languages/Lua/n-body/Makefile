compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  nbody.lua-2.lua 50000000" Lua n-body

run:
	lua  nbody.lua-2.lua 50000000

mem:
	/usr/bin/time -v  lua  nbody.lua-2.lua 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  lua  nbody.lua-2.lua 50000000
