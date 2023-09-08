compile:
	~/.dotnet/dotnet new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/mandelbrot.csproj.nuget.g.props
	cp mandelbrot.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./bin/Release/net7.0/mandelbrot 16000" CSharp mandelbrot

run:
	./bin/Release/net7.0/mandelbrot 16000

mem:
	/usr/bin/time -v ./bin/Release/net7.0/mandelbrot 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./bin/Release/net7.0/mandelbrot 16000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj