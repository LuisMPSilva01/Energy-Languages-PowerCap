compile:
	~/.dotnet/dotnet new console --force
	cp fasta.csharpcore-5.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/fasta 25000000" CSharp fasta

run:
	./bin/Release/net7.0/fasta 25000000

mem:
	/usr/bin/time -v  ./bin/Release/net7.0/fasta 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./bin/Release/net7.0/fasta 25000000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj