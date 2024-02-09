NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt" Lua regex-redux $(NTIMES)

run:
	/usr/local/bin/lua regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt


mem:
	/usr/bin/time -v lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes lua  regexredux.lua-2.lua 0 < ../../../inputs/regexredux-input5000000.txt
