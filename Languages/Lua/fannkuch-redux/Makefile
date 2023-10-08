compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  fannkuchredux.lua 12" Lua fannkuch-redux

run:
	lua  fannkuchredux.lua 12

mem:
	/usr/bin/time -v lua  fannkuchredux.lua 12

valgrind:
	valgrind --tool=massif --stacks=yes lua  fannkuchredux.lua 12
