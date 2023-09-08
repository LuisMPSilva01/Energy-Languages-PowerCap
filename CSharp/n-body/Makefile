compile:
	~/.dotnet/dotnet new console --force
	cp nbody.csharpcore-9.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/n-body 50000000" CSharp n-body

run:
	./bin/Release/net7.0/n-body 50000000

mem:
	/usr/bin/time -v  ./bin/Release/net7.0/n-body 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./bin/Release/net7.0/n-body 50000000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj