compile:
	~/.dotnet/dotnet new console -lang "F#" --force
	cp knucleotide.fsharpcore-6.fsharpcore Program.fs
	~/.dotnet/dotnet add package Microsoft.Experimental.Collections --prerelease
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/bin/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt" FSharp k-nucleotide

run:
	/usr/bin/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v  /usr/bin/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/dotnet ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.fs
	rm -f *.fsproj