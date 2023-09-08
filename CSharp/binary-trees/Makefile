compile:
	~/.dotnet/dotnet new console --force
	cp binarytrees.csharpcore-7.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/binary-trees.dll 21" CSharp binary-trees

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/binary-trees.dll 21

mem:
	/usr/bin/time -v  ~/.dotnet/dotnet ./bin/Release/net7.0/binary-trees.dll 21

valgrind:
	valgrind --tool=massif --stacks=yes  ~/.dotnet/dotnet ./bin/Release/net7.0/binary-trees.dll 21

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj