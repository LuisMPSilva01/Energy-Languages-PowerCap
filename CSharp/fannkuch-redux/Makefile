compile:
	~/.dotnet/dotnet new console --force
	cp fannkuchredux.csharpcore-8.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained 

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/fannkuch-redux 12" CSharp fannkuch-redux

run:
	./bin/Release/net7.0/fannkuch-redux 12

mem:
	/usr/bin/time -v  ./bin/Release/net7.0/fannkuch-redux 12

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj
valgrind:
	valgrind --tool=massif --stacks=yes ./bin/Release/net7.0/fannkuch-redux 12
