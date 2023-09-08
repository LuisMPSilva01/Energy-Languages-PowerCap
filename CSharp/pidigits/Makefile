compile:
	~/.dotnet/dotnet new console --force
	cp pidigits.csharpcore-5.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/pidigits 10000" CSharp pidigits

run:
	./bin/Release/net7.0/pidigits 10000

mem:
	./bin/Release/net7.0/pidigits 10000

valgrind:
	valgrind --tool=massif --stacks=yes ./bin/Release/net7.0/pidigits 10000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj