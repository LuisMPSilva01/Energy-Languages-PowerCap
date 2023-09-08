compile:
	~/.dotnet/dotnet new console --force
	cp knucleotide.csharpcore Program.cs
	~/.dotnet/dotnet add reference ../../corefxlab/src/Microsoft.Experimental.Collections
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < knucleotide-input25000000.txt" CSharp k-nucleotide

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ~/.dotnet/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ~/.dotnet/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < knucleotide-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj