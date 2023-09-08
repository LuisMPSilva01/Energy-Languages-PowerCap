compile:
	~/.dotnet/dotnet new console --force
	cp pidigits.csharpcore-5.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/pidigits.dll 10000" CSharp pidigits

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/pidigits.dll 10000

mem:
	~/.dotnet/dotnet ./bin/Release/net7.0/pidigits.dll 10000

valgrind:
	valgrind --tool=massif --stacks=yes ~/.dotnet/dotnet ./bin/Release/net7.0/pidigits.dll 10000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj