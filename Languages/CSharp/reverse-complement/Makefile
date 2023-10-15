compile:
	~/.dotnet/dotnet new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/reverse-complement.csproj.nuget.g.props
	cp revcomp.csharpcore-8.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt" CSharp reverse-complement

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v  ~/.dotnet/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes   ~/.dotnet/dotnet ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj