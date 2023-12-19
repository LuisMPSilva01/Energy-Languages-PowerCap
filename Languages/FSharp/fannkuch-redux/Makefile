NTIMES=1

export NTIMES

compile:
	~/.dotnet/dotnet new console -lang "F#" --force
	cp fannkuchredux.fsharpcore-6.fsharpcore Program.fs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/fannkuch-redux.dll 12" FSharp fannkuch-redux $(NTIMES)

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/fannkuch-redux.dll 12

mem:
	/usr/bin/time -v  /usr/bin/dotnet ./bin/Release/netcoreapp1.1/tmp.dll 12

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/dotnet ./bin/Release/netcoreapp1.1/tmp.dll 12

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.fs
	rm -f *.fsproj