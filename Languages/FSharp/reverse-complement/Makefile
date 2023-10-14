compile:
	~/.dotnet/dotnet new console -lang "F#" --force
	cp revcomp.fsharpcore-4.fsharpcore Program.fs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/bin/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../inputs/revcomp-input25000000.txt" FSharp reverse-complement

run:
	/usr/bin/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v  /usr/bin/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../inputs/revcomp-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.fs
	rm -f *.fsproj