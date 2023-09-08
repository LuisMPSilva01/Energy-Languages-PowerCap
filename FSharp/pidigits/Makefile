compile:
	~/.dotnet/dotnet new console -lang "F#" --force
	cp pidigits.fsharpcore-6.fsharpcore Program.fs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/bin/dotnet ./bin/Release/net7.0/pidigits.dll 10000" FSharp pidigits

run:
	/usr/bin/dotnet ./bin/Release/net7.0/pidigits.dll 10000

mem:
	/usr/bin/time -v /usr/bin/dotnet ./bin/Release/net7.0/pidigits.dll 10000

valgrind:
	valgrind --tool=massif --stacks=yes ./usr/bin/dotnet ./bin/Release/net7.0/pidigits.dll 10000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.fs
	rm -f *.fsproj