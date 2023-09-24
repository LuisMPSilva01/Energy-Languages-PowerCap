compile:

xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  pidigits.lua-5.lua 10000" Lua pidigits

run:
	lua  pidigits.lua-5.lua 10000

mem:
	/usr/bin/time -v lua  pidigits.lua-5.lua 10000


valgrind:
	valgrind --tool=massif --stacks=yes lua  pidigits.lua-5.lua 10000

