compile:
	~/.dotnet/dotnet new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/reverse-complement.csproj.nuget.g.props
	cp revcomp.csharpcore-8.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/reverse-complement 0 < ../../revcomp-input25000000.txt" CSharp reverse-complement

run:
	./bin/Release/net7.0/reverse-complement 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v  ./bin/Release/net7.0/reverse-complement 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes   ./bin/Release/net7.0/reverse-complement 0 < ../../revcomp-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj