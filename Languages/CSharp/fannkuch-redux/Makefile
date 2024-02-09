NTIMES=20

export NTIMES

compile:
	~/.dotnet/dotnet new console --force
	cp fannkuchredux.csharpcore-8.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/fannkuch-redux.dll 12" CSharp fannkuch-redux $(NTIMES)

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/fannkuch-redux.dll 12

mem:
	/usr/bin/time -v  ~/.dotnet/dotnet ./bin/Release/net7.0/fannkuch-redux.dll 12

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj
valgrind:
	valgrind --tool=massif --stacks=yes ~/.dotnet/dotnet ./bin/Release/net7.0/fannkuch-redux.dll 12
