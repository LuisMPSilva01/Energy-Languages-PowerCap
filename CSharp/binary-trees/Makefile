compile:
	~/.dotnet/dotnet new console --force
	cp binarytrees.csharpcore-7.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/binary-trees 21" CSharp binary-trees

run:
	./bin/Release/net7.0/binary-trees 21

mem:
	/usr/bin/time -v  ./bin/Release/net7.0/binary-trees 21

valgrind:
	valgrind --tool=massif --stacks=yes  ./bin/Release/net7.0/binary-trees 21

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj