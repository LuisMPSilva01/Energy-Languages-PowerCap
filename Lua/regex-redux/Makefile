compile:

xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  regexredux.lua-2.lua 0 < ../../regexredux-input50000.txt" Lua regex-redux

run:
	lua  regexredux.lua-2.lua 0 < ../../regexredux-input50000.txt

mem:
	/usr/bin/time -v lua  regexredux.lua-2.lua 0 < ../../regexredux-input50000.txt

valgrind:
	valgrind --tool=massif --stacks=yes lua  regexredux.lua-2.lua 0 < ../../regexredux-input50000.txt
